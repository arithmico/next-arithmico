use std::{
    collections::HashSet,
    fs::{File, OpenOptions, remove_file},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Parser;
use glob::{Paths, glob};

#[derive(Parser, Debug)]
struct Arguments {
    #[arg(short, long, value_parser, value_delimiter = ',')]
    include: Vec<String>,

    #[arg(short, long, value_parser, value_delimiter = ',')]
    exclude: Vec<String>,

    #[arg(short, long)]
    output: String,
}

pub fn main() -> ExitCode {
    let arguments = Arguments::parse();

    match run(&arguments) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            println!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: &Arguments) -> Result<(), String> {
    let paths = get_included_paths(
        &arguments.include,
        &arguments.exclude,
        &arguments.output,
    )?;
    let chunks: String = read_files(&paths)?.join("\n");
    let output_path = Path::new(&arguments.output);
    if output_path.exists() {
        remove_file(output_path).map_err(|_| {
            format!(
                "Failed to delete file {}",
                output_path.to_str().unwrap_or("")
            )
        })?;
    }
    File::create(&arguments.output)
        .map_err(|_| String::from("Failed to create output file"))?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&arguments.output)
        .map_err(|_| String::from("Failed to open output file"))?;

    file.write_all(chunks.as_bytes())
        .map_err(|_| String::from("Failed to write output file"))
}

fn get_included_paths(
    include_patterns: &[String],
    excluded_patterns: &[String],
    output: &str,
) -> Result<HashSet<PathBuf>, String> {
    let include_paths = get_paths_for_pattern_list(include_patterns)?;
    let exclude_paths = get_paths_for_pattern_list(excluded_patterns)?
        .iter()
        .filter_map(|item| item.to_str())
        .map(|item| item.to_string())
        .collect::<HashSet<_>>();

    let output = Path::new(output)
        .canonicalize()
        .map_err(|_| String::from("Failed to normalize output path"))?
        .to_str()
        .ok_or(String::from("Failed to serialize normalized output path"))?
        .to_string();

    let mut result_paths = HashSet::new();
    for include_path in include_paths {
        let Some(path) = include_path.to_str() else {
            continue;
        };
        let path = path.to_string();
        if exclude_paths.contains(&path) {
            continue;
        }
        if path == output {
            continue;
        }
        let extension = include_path
            .extension()
            .map(|s| s.to_str())
            .flatten()
            .map(|s| s.to_lowercase())
            .unwrap_or(String::new());
        if extension != "css" {
            println!(
                "Skipping non css file {}",
                include_path.to_str().unwrap_or("")
            );
            continue;
        }
        result_paths.insert(include_path);
    }

    Ok(result_paths)
}

fn get_paths_for_pattern_list(
    patterns: &[String],
) -> Result<HashSet<PathBuf>, String> {
    let mut paths = HashSet::<PathBuf>::new();
    for pattern in patterns {
        for path in get_paths_for_pattern(pattern)? {
            let path = path.map_err(|_| String::from("IO Error"))?;
            if let Ok(path) = path.canonicalize() {
                paths.insert(path);
            } else {
                println!(
                    "Warning: Failed to normalize path {}",
                    path.to_str().unwrap_or("")
                );
            }
        }
    }
    Ok(paths)
}

fn get_paths_for_pattern(pattern: &str) -> Result<Paths, String> {
    glob(pattern)
        .map_err(|_| format!("Failed to parse glob pattern \"{}\"", pattern))
}

fn read_files(paths: &HashSet<PathBuf>) -> Result<Vec<String>, String> {
    let mut chunks = Vec::<String>::new();
    for path in paths {
        let path_str = path.to_str().unwrap_or("none");
        let mut file = File::open(path.as_path())
            .map_err(|_| format!("Failed to open file {}", path_str))?;
        let mut chunk = String::new();
        file.read_to_string(&mut chunk)
            .map_err(|_| format!("Failed to read file {}", path_str))?;
        chunks.push(chunk);
    }
    Ok(chunks)
}
