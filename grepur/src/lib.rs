use clap::Parser;
use regex::{Captures, Regex};
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern to search
    #[arg(short, long)]
    pattern: String,

    /// Name of the directory to search in
    #[arg(short, long)]
    dir: String,
}

fn read_lines(filename: &Path) -> Vec<String> {
    let result = read_to_string(filename);
    match result {
        Ok(res) => {
            return res.lines().map(String::from).collect();
        }
        Err(_) => {
            return Vec::new();
        }
    }
}

// pub fn search() -> Vec<String> {
pub fn search() {
    let args = Args::parse();

    // let results: Vec<String> = Vec::new();
    for entry in WalkDir::new(args.dir) {
        let entry = entry.unwrap();
        // Open the file
        let vec_str = read_lines(entry.path());
        for vs in vec_str {
            // Regex
            let re = Regex::new(args.pattern.as_str()).unwrap();
            let capture: Option<Captures> = re.captures(vs.as_str());
            match capture {
                Some(capt) => match capt.get(0) {
                    Some(_) => {
                        println!("{}: {}", entry.path().display(), vs);
                        // results.push(vs);
                    }
                    None => {}
                },
                None => {}
            }
        }
    }

    // results
}
