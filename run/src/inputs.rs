use colored::*;
use reqwest::blocking::ClientBuilder;
use std::{fs, path::PathBuf};

const AOC_INPUTS_CACHE_PATH: &str = ".aoc_inputs";

pub fn get_puzzle_input_path(day: usize) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(AOC_INPUTS_CACHE_PATH);
    path.push(format!("puzzle_{:02}.txt", day));
    path
}

pub fn download_puzzle_input(day: usize, session_cookie: &str) {
    // ensure puzzle input hasn't been downloaded yet
    let puzzle_input_path = get_puzzle_input_path(day);
    if fs::metadata(&puzzle_input_path).is_ok() {
        panic!(
            "it looks like the requested puzzle input already exists: {}",
            puzzle_input_path.to_string_lossy()
        );
    }

    // fetch puzzle input
    let url: String = format!("https://adventofcode.com/2023/day/{}/input", day);
    println!("starting download... ({})", &url);
    let session_cookie = format!("session={}", session_cookie);
    let client = ClientBuilder::new().build().unwrap();
    let puzzle_input = client
        .get(url)
        .header("Cookie", session_cookie)
        .send()
        .expect("failed to send request")
        .text()
        .expect("failed to parse response");

    // write puzzle input to file
    let _ = fs::create_dir_all(AOC_INPUTS_CACHE_PATH);
    fs::write(&puzzle_input_path, puzzle_input).expect("failed to write file");

    let download_success_message = format!("successfully downloaded puzzle input!").bright_green();
    println!("{}", download_success_message);
}

pub fn list_stored_puzzle_inputs() {
    fs::read_dir(AOC_INPUTS_CACHE_PATH)
        .and_then(|dir| {
            for entry in dir {
                let entry = entry.expect("failed to read directory entry");
                let metadata = entry
                    .metadata()
                    .expect("failed to read directory entry metadata");
                if metadata.is_file() {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
            Ok(())
        })
        .expect("failed to read directory");
}

pub fn read_puzzle_input(day: usize) -> Option<String> {
    let puzzle_input_path = get_puzzle_input_path(day);
    match fs::read_to_string(puzzle_input_path) {
        Err(_) => None,
        Ok(content) => Some(content),
    }
}

pub fn read_puzzle_example_inputs(day: usize, part: usize) -> Vec<(String, String)> {
    let expected_file_name_start = format!("puzzle_{:02}_part_{}_", day, part);
    let mut example_inputs = vec![];
    fs::read_dir(AOC_INPUTS_CACHE_PATH).and_then(|dir| {
        for entry in dir {
            let entry = entry.expect("failed to read directory entry");
            let metadata = entry
                .metadata()
                .expect("failed to read directory entry metadata");
            if metadata.is_file() {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().expect("failed to read file name");
                if file_name.starts_with(&expected_file_name_start) && file_name.ends_with(".txt") {
                    let note = &file_name[17..(file_name.len() - 4)];
                    match fs::read_to_string(entry.path()) {
                        Err(_) => panic!("failed to read file"),
                        Ok(content) => example_inputs.push((note.to_owned(), content)),
                    }
                }
            }
        }
        Ok(())
    }).expect("failed to read directory");
    example_inputs
}
