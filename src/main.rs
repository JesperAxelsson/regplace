extern crate clap;

use clap::{Arg, App};

use std::io;
use std::fs::{self, DirEntry, rename};
//use std::path::Path;

use globset::Glob;
use globset::GlobMatcher;
use regex::Regex;


fn main() -> io::Result<()> {
       let matches = App::new("Regex replace")
                        .version("1.0")
                        .author("Jesper Axelsson <jesperaxe@gmail.com>")
                        .about("Rename files with regex")
                        .arg(Arg::with_name("regex pattern")
                            .help("Sets regex capture pattern")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("regex replace pattern")
                            .help("Sets the regex replace pattern")
                            .required(true)
                            .index(2))
                        .arg(Arg::with_name("path")
                            .short("p")
                            .long("path")
                            .value_name("PATH")
                            .default_value(".")
                            .takes_value(true)
                            .help("Path to files"))
                        .arg(Arg::with_name("file pattern")
                            .short("f")
                            .long("file-pattern")
                            .value_name("FILE_PATTERN")
                            .default_value("*")
                            .takes_value(true)
                            .help("Sets the input files filter"))                            
                        .arg(Arg::with_name("dry-run")
                            .short("d")
                            .long("dry-run")
                            .help("Perform a dry tun to see changes"))
                        .arg(Arg::with_name("overwrite")
                            .short("o")
                            .long("overwrite")
                            .help("Overwrite existing files"))
                        .arg(Arg::with_name("recursive")
                            .short("r")
                            .help("Go into sub directories"))
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Sets the level of verbosity"))
                        .get_matches();

    let regex_pattern = matches.value_of("regex pattern").unwrap();
    let regex_replace_pattern = matches.value_of("regex replace pattern").unwrap();

    let path = matches.value_of("path").unwrap_or(".");
    let file_pattern = matches.value_of("FILE_PATTERN").unwrap_or_default();

    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");
    let overwrite = matches.is_present("overwrite");
    let recursive = matches.is_present("recursive");

    if verbose {
        println!("Using input file: {}", file_pattern);
        println!("Using regex: {}", regex_pattern);
        println!("Using regex replace: {}", regex_replace_pattern);
        println!("Using path: {}", path);
    }

    let glob = Glob::new(file_pattern).expect("Wrong glob file pattern").compile_matcher();
    let mut files = Vec::new();

    get_files(recursive, &mut files, path, &glob);

    for f in files.iter() {
        if verbose {
            println!("{:?}", f);
        }
    }

    let regex = Regex::new(regex_pattern).expect("Failed to compile file regex");

    for f in files {
        let original_name = String::from(f.file_name().to_str().unwrap());
        
        if regex.is_match(&original_name) {
            let new_name = regex.replace_all(&original_name, regex_replace_pattern);
            let mut new_path = f.path().clone();
            new_path.set_file_name(new_name.into_owned());
            
            if !new_path.exists() {
                println!("{} --> {}", f.path().display(), new_path.display());
            } else {
                println!("{} --> {} already exists", f.path().display(), new_path.display());
            }

            if !dry_run && (!new_path.exists() || overwrite) {
                // Change the names!
                rename(f.path(), new_path)?;
            }
        }
    }

    Ok(())
}


fn get_files(recursive: bool, files: &mut Vec<DirEntry>, path: &str, glob: &GlobMatcher) {
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed dir");
        let path = entry.path();
        if path.is_file() && glob.is_match(&path) {
            files.push(entry);
        } else if path.is_dir() && recursive {
            get_files(recursive, files, path.to_str().unwrap(), glob);
        }        
    }
}
