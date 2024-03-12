use std::fs;
use std::path::Path;
use std::io::prelude::*;

struct FileIndex {
    name: String,
    content: String,
}

fn main() {
    let path = Path::new("Engines");
    let mut files: Vec<FileIndex> = Vec::new();

    read_files(&path, &mut files);

    print_first_word(&files);
}

fn read_files(path: &Path, files: &mut Vec<FileIndex>) {
    if path.is_file() {
        let mut file = fs::File::open(&path)
            .unwrap_or_else(|err| panic!("Unable to open file {}: {}", path.display(), err));
        let mut content = String::new();
        file.read_to_string(&mut content)
            .unwrap_or_else(|err| panic!("Unable to read file {}: {}", path.display(), err));
        files.push(FileIndex {
            name: path.to_string_lossy().into_owned(),
            content,
        });
    } else if path.is_dir() {
        let entries = fs::read_dir(path)
            .unwrap_or_else(|err| panic!("Unable to read directory {}: {}", path.display(), err));
        for entry in entries {
            let entry = entry.expect("Unable to read directory entry");
            read_files(&entry.path(), files);
        }
    }
}

fn print_first_word(files: &Vec<FileIndex>) {
    for file in files {
        let first_word = file.content.split_whitespace().next();
        match first_word {
            Some(word) => println!("First word in file {}: {}", file.name, word),
            None => println!("No words in file {}", file.name),
        }
    }
}