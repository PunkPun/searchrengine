use std::fs;
use std::io::{self, Write};
use std::i32;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

struct FileIndex {
    filename: String,
    link: String,
}

pub fn run_boolean_bigram_search_engine(path: &Path) {
    let mut files: Vec<FileIndex> = Vec::new();
    let mut indices: HashMap<String, HashSet<i32>> = HashMap::new();
	let mut bigrams: HashMap<String, HashSet<i32>> = HashMap::new();

    let mut file_count = 0;
    read_files(&path, &mut files, &mut indices, &mut bigrams, &mut file_count);

    loop {
        print!("Enter a phrase: ");
        io::stdout().flush().unwrap();
        let mut phrase = String::new();
        io::stdin().read_line(&mut phrase).unwrap();
        let phrase = phrase.trim();

        if phrase.is_empty() {
            break
        }

		let file_indices = search_indices_with_bigrams(&indices, &bigrams, phrase);
        if file_indices.is_empty() {
            println!("No results found for phrase: {}", phrase);
        } else {
            for index in file_indices {
                if let Some(file) = files.get(index as usize) {
                    println!("File: {}, link: {}", file.filename, file.link);
                }
            }
        }
    }
}

fn sanitize_word(word: &str) -> String {
    word.to_lowercase()
}

fn search_indices_with_bigrams(indices: &HashMap<String, HashSet<i32>>, bigrams: &HashMap<String, HashSet<i32>>,
		phrase: &str) -> Vec<i32> {
	let words: Vec<&str> = phrase.split_whitespace().collect();

	let mut unique_results: HashSet<i32> = HashSet::new();
	let mut dedup_bigram_results: Vec<i32> = Vec::new();

	for result in search_bigrams_and(bigrams, words.clone()) {
		if unique_results.insert(result) {
			dedup_bigram_results.push(result);
		}
	}

	for result in search_indices_and(indices, words.clone()) {
		if unique_results.insert(result) {
			dedup_bigram_results.push(result);
		}
	}

	dedup_bigram_results
}

fn search_indices_and(indices: &HashMap<String, HashSet<i32>>, words: Vec<&str>) -> HashSet<i32> {
    if let Some(first_word) = words.first() {
        let mut result = indices.get(&sanitize_word(first_word)).cloned().unwrap_or_default();

        for word in words.iter().skip(1) {
            if let Some(indices_set) = indices.get(&sanitize_word(word)) {
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

fn search_bigrams_and(bigrams: &HashMap<String, HashSet<i32>>, words: Vec<&str>) -> HashSet<i32> {
	let bi_grams: Vec<(&str, &str)> = words.windows(2).map(|w| (w[0], w[1])).collect();
	if let Some(first_bigram) = bi_grams.first() {
		let bigram_str = format!("{} {}", first_bigram.0, first_bigram.1);
        let mut result = bigrams.get(&sanitize_word(&*bigram_str)).cloned().unwrap_or_default();

        for bigram in bi_grams.iter().skip(1) {
			let bigram_str = format!("{} {}", bigram.0, bigram.1);
            if let Some(indices_set) = bigrams.get(&sanitize_word(&*bigram_str)) {
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

fn read_files(path: &Path, files: &mut Vec<FileIndex>, indices: &mut HashMap<String, HashSet<i32>>,
		bigrams: &mut HashMap<String, HashSet<i32>>, file_count: &mut i32) {
    if path.is_file() {
		let mut file = fs::File::open(&path)
            .unwrap_or_else(|err| panic!("Unable to open file {}: {}", path.display(), err));

		let mut text = String::new();
        file.read_to_string(&mut text)
            .unwrap_or_else(|err| panic!("Unable to read file {}: {}", path.display(), err));

		let mut lines = text.lines();
		let link = lines.next().unwrap_or("").to_string();

		let words: Vec<String> = lines.flat_map(|line| line.split_whitespace().map(sanitize_word)).collect();

		for word in words.iter() {
			indices.entry(word.to_string()).or_insert_with(HashSet::new).insert(*file_count);
		}

        for bigram in words.windows(2).map(|w| (&w[0], &w[1])) {
            let bigram_str = format!("{} {}", *bigram.0, *bigram.1);
            bigrams.entry(bigram_str.to_string()).or_insert_with(HashSet::new).insert(*file_count);
        }

        let name = path.strip_prefix("Engines")
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();

        files.push(FileIndex {
            filename: name,
            link,
        });

		*file_count += 1;
    } else if path.is_dir() {
        let entries = fs::read_dir(path)
            .unwrap_or_else(|err| panic!("Unable to read directory {}: {}", path.display(), err));
        for entry in entries {
            let entry = entry.expect("Unable to read directory entry");
            read_files(&entry.path(), files, indices, bigrams, file_count);
        }
    }
}