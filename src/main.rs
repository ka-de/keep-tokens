/*
 * This code designed to process a directory of .txt files.
 *
 * It defines a list of "keep tokens" that should be retained in the files.
 * For each .txt file found, it reads the content, splits it into tags
 * (separated by commas) and sentences (after the first comma-separated list).
 *
 * It then filters out tags that are not in the "keep tokens" list, unless they
 * appear in the sentences. Finally, it writes a new version of the file with the
 * format: keep_tokens ||| filtered_tags, sentences.
 */

use std::fs::{ read_dir, File };
use std::io::{ BufRead, BufReader, Write };
use std::path::Path;

fn main() {
    let keep_tokens = vec!["feral", "weasel", "photography \\(artwork\\)"];
    let directory = Path::new("E:\\training_dir_staging");
    let mut files = Vec::new();
    println!("Searching for .txt files in directory: {}", directory.display());
    get_txt_files(directory, &mut files);
    println!("Found {} .txt files", files.len());

    for file in &files {
        println!("Processing file: {}", file);
        let content = read_file(file);
        let (tags, sentences) = split_content(&content);
        let filtered_tags: Vec<_> = tags
            .into_iter()
            .filter(|tag| (!keep_tokens.contains(tag) || sentences.contains(tag)))
            .collect();
        let new_content = format!(
            "{} ||| {}, {}",
            keep_tokens.join(", "),
            filtered_tags.join(", "),
            sentences
        );
        write_file(file, &new_content);
        println!("Wrote new content to file: {}", file);
    }
}

fn get_txt_files(dir: &Path, files: &mut Vec<String>) {
    if dir.is_dir() {
        // Skip the .git directory
        if dir.ends_with(".git") {
            return;
        }

        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                println!("Recursing into directory: {}", path.display());
                get_txt_files(&path, files);
            } else if
                path.extension().unwrap_or_default() == "txt" &&
                !path.file_name().unwrap().to_string_lossy().contains("-sample-prompts.txt") &&
                path.file_name().unwrap().to_string_lossy() != "sample-prompts.txt"
            {
                files.push(path.to_string_lossy().into_owned());
                println!("Found file: {}", path.display());
            }
        }
    }
}

fn read_file(file: &str) -> String {
    println!("Reading file: {}", file);
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<String, _>>().unwrap()
}

fn split_content(content: &str) -> (Vec<&str>, &str) {
    let split: Vec<_> = content.split("., ").collect();
    let tags: Vec<_> = split[0].split(',').collect();
    let sentences = split.get(1).unwrap_or(&"");
    (tags, sentences.trim())
}

fn write_file(file: &str, content: &str) {
    println!("Writing new content to file: {}", file);
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
