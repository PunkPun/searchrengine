use std::fs;
use std::io::{self, Write};
use std::i32;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

struct FileIndex {
    name: String,
    link: String,
}

fn main() {
    let path = Path::new("Engines");
    let mut files: Vec<FileIndex> = Vec::new();
    let mut indices: HashMap<String, HashSet<i32>> = HashMap::new();

    let mut file_count = 0;
    read_files(&path, &mut files, &mut indices, &mut file_count);

    loop {
        print!("Enter a phrase: ");
        io::stdout().flush().unwrap();
        let mut phrase = String::new();
        io::stdin().read_line(&mut phrase).unwrap();
        let phrase = phrase.trim();

        if phrase.is_empty() {
            break
        }

        let file_indices = search_indices_and(&indices, phrase);
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

fn sanitize_word(word: &str) -> String {
    word.to_lowercase()
}

fn search_indices_and(indices: &HashMap<String, HashSet<i32>>, phrase: &str) -> HashSet<i32> {
    let words: Vec<String> = phrase.split_whitespace().map(sanitize_word).collect();
    if let Some(first_word) = words.first() {
        let mut result = indices.get(first_word).cloned().unwrap_or_default();

        for word in words.iter().skip(1) {
            if let Some(indices_set) = indices.get(word) {
                result = result.intersection(indices_set).cloned().collect();
            } else {
                return HashSet::new();
            }
        }

        result
    } else {
        HashSet::new()
    }
}

fn read_files(path: &Path, files: &mut Vec<FileIndex>, indices: &mut HashMap<String, HashSet<i32>>, file_count: &mut i32) {
    if path.is_file() {
        *file_count += 1;
        let mut file = fs::File::open(&path)
            .unwrap_or_else(|err| panic!("Unable to open file {}: {}", path.display(), err));
        let mut lines = String::new();
        file.read_to_string(&mut lines)
            .unwrap_or_else(|err| panic!("Unable to read file {}: {}", path.display(), err));
        let mut lines = lines.lines();
        let link = lines.next().unwrap_or("").to_string();
        
        for line in lines {
            for word in line.split_whitespace() {
                let word = sanitize_word(word);
                let entry = indices.entry(word).or_insert(HashSet::new());
                entry.insert(*file_count);
            }
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
            read_files(&entry.path(), files, indices, file_count);
        }
    }
}