use std::fs;
use std::io::{self, Write};
use std::i32;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

struct FileIndex {
    name: String,
    link: String,
}

struct Index {
    idf: f32,
    tf: HashMap<i32, f32>,
}

pub fn run_vector_search_engine(path: &Path) {
    let mut files: Vec<FileIndex> = Vec::new();
    let mut indices: HashMap<String, Index> = HashMap::new();
    let mut file_count = 0;
    let log = 4.0;
    read_files(&path, &mut files, &mut indices, &mut file_count, log);

    for index in indices.values_mut() {
        let doc_count = index.tf.len() as f32;
        index.idf = (file_count as f32 / doc_count).log(log);
    }

    loop {
        print!("Enter a phrase: ");
        io::stdout().flush().unwrap();
        let mut phrase = String::new();
        io::stdin().read_line(&mut phrase).unwrap();
        let phrase = phrase.trim();

        if phrase.is_empty() {
            break
        }

		let file_indices = search_indices(&indices, phrase);
        if file_indices.is_empty() {
            println!("No results found for phrase: {}", phrase);
        } else {
            for index in file_indices {
                if let Some(file) = files.get(index as usize) {
                    println!("File: {}, link: {}", file.name, file.link);
                }
            }
        }
    }
}

fn search_indices(indices: &HashMap<String, Index>, phrase: &str) -> Vec<i32> {
    
    let mut result = Vec::new();
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let mut query: HashMap<i32, f32> = HashMap::new();
    
    for word in words {
        let word = sanitize_word(word);
        let index = indices.get(&word);
        index.map(|index| {
            let idf = index.idf;
            for (file, tf) in &index.tf {
                let tf = *tf;
                let score = query.entry(*file).or_insert(0.0);
                *score += tf * idf;
            }
        });
    }

    let mut query: Vec<_> = query.iter().collect();
    query.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (file, _) in query {
        result.push(*file);
    }

    result
}

fn sanitize_word(word: &str) -> String {
    word.to_lowercase()
}

fn read_files(path: &Path, files: &mut Vec<FileIndex>, indices: &mut HashMap<String, Index>, file_count: &mut i32, log: f32) {
    if path.is_file() {
        *file_count += 1;
        let mut file = fs::File::open(&path)
            .unwrap_or_else(|err| panic!("Unable to open file {}: {}", path.display(), err));

        let mut text = String::new();
        file.read_to_string(&mut text)
            .unwrap_or_else(|err| panic!("Unable to read file {}: {}", path.display(), err));

        let mut lines = text.lines();
        let link = lines.next().unwrap_or("").to_string();

        let mut word_count: HashMap<String, u32> = HashMap::new();
        for line in lines {
            for word in line.split_whitespace() {
                let word = sanitize_word(word);
                let count = word_count.entry(word).or_insert(0);
                *count += 1;
            }
        }

        for (word, count) in word_count {
            let tf = 1.0 + (count as f32).log(log);
            indices.entry(word.to_string())
                .or_insert(Index { idf: 0.0, tf: HashMap::new() })
                .tf.insert(*file_count, tf);
        }
        
        let name = path.strip_prefix("Engines")
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();

        files.push(FileIndex {
            name,
            link,
        });
    } else if path.is_dir() {
        let entries = fs::read_dir(path)
            .unwrap_or_else(|err| panic!("Unable to read directory {}: {}", path.display(), err));
        for entry in entries {
            let entry = entry.expect("Unable to read directory entry");
            read_files(&entry.path(), files, indices, file_count, log);
        }
    }
}