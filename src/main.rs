#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate regex;

use std::fs;
use regex::Regex;
use std::env;

pub mod config;
use config::{Config, Match};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Specify the configuration file and the root folder");
    }

    let config = {
        let file = fs::File::open(&args[1]).expect("Cannot open configuration file");
        serde_json::from_reader(file).expect("Configuration file syntax error")
    };

    let folder = &args[2];

    scan_first_level(&config, folder).unwrap();
}

fn scan_first_level(config: &Config, folder: &str) -> Result<(), std::io::Error> {
    for de_ in fs::read_dir(folder)? {
        let de_path = de_?.path();
        let path = std::path::Path::new(&de_path);

        println!("Processing {:?}", path);

        if let Some(file) = path.to_str() {
            if path.is_file() {
                if let Some(m) = matches(file, config) {
                    if extension_matches(config, path) {
                        println!("\tfile skipped: extension matches the incomplete one");
                    } else {
                        let mut pb = std::path::PathBuf::new();
                        pb.push(&m.destination);
                        pb.push(path.file_name().unwrap());

                        println!("\tmove as {:?}", pb.as_path());
                        std::fs::rename(file, pb).expect("Cannot move file");
                    }
                } else {
                    println!("\tfile does not match any Match. Ignoring");
                }
            } else if let Some(m) = matches(file, config) {
                if scan_for_incomplete(config, file)? {
                    println!("\tfolder skipped due to incomplete file found");
                } else {
                    let mut pb = std::path::PathBuf::new();
                    pb.push(&m.destination);
                    pb.push(path.file_name().unwrap());

                    println!("\tmove as {:?}", pb.as_path());
                    std::fs::rename(file, pb).expect("Cannot move folder");
                }
            } else {
                println!("\tfolder does not match any Match. Ignoring");
            }
        } else {
            panic!("cannot read path from file");
        }
    }

    Ok(())
}

fn matches<'a>(file: &str, config: &'a Config) -> Option<&'a Match> {
    for m in &config.matches {
        let re = Regex::new(&m.regex).expect("Invalid regular expression");

        if re.is_match(file) {
            println!("\t{} match found with {:?}", file, m);
            return Some(m);
        }
    }

    None
}

fn scan_for_incomplete(config: &Config, folder: &str) -> Result<bool, std::io::Error> {
    for de_ in fs::read_dir(folder)? {
        let de_path = de_?.path();

        let path = std::path::Path::new(&de_path);

        if path.is_file() {
            if extension_matches(config, path) {
                return Ok(true);
            }
        } else if let Some(inner_path) = path.to_str() {
            if scan_for_incomplete(config, inner_path)? {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn extension_matches(config: &Config, path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext) = ext.to_str() {
            if ext == config.skipextension {
                return true;
            }
        }
    }
    false
}
