#![allow(dead_code)]

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

mod binary_search;
mod coin_change;
mod factorial;
mod graph;
mod heap;
mod knn;
mod levenshtein_distance;
mod merge_sort;
mod queue;
mod quick_sort;
mod stack;

fn main() -> io::Result<()> {
    let path = Path::new("./src");
    let files = fs::read_dir(path)?;
    let mut links = vec![
        "<!-- This file was generated -->".to_string(),
        "# algos".to_string(),
    ];

    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name_str = file_name.to_string_lossy();
        let file_stem = file_name_str.split('.').next().unwrap_or(&file_name_str);

        if file_stem.to_lowercase() != "main" {
            links.push(format!("- [{}](src/{})", file_stem, file_name_str));
        }
    }

    let mut file = File::create("README.md")?;
    for link in links {
        writeln!(file, "{}", link)?;
    }

    Ok(())
}
