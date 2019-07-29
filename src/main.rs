extern crate clap;

use clap::{Arg, App};
// use walkdir::WalkDir;
// use glob::glob;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

use globset::Glob;
use regex::Regex;


fn main() {
       let matches = App::new("Regex replace")
                          .version("1.0")
                          .author("Jesper Axelsson <jesperaxe@gmail.com>")
                          .about("Rename files with regex")
                          .arg(Arg::with_name("file pattern")
                               .help("Sets the input files to use")
                               .required(true)
                               .index(1))
                        .arg(Arg::with_name("regex pattern")
                            .help("Sets rexex capture pattern")
                            .required(true)
                            .index(2))
                        .arg(Arg::with_name("regex replace pattern")
                            .help("Sets the regex replace pattern")
                               .required(true)
                               .index(3))
                        //   .arg(Arg::with_name("v")
                        //        .short("v")
                        //        .multiple(true)
                            //    .help("Sets the level of verbosity"))
                          
                          .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let file_pattern = matches.value_of("file pattern").unwrap();
    let regex_pattern = matches.value_of("regex pattern").unwrap();
    let regex_replace_pattern = matches.value_of("regex replace pattern").unwrap();
    println!("Using input file: {}", file_pattern);
    println!("Using regex: {}", regex_pattern);
    println!("Using regex replace: {}", regex_replace_pattern);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
        // match matches.occurrences_of("v") {
        //     0 => println!("No verbose info"),
        //     1 => println!("Some verbose info"),
        //     2 => println!("Tons of verbose info"),
        //     3 | _ => println!("Don't be crazy"),
        // }


    // for entry in glob(file_pattern).expect("Failed to read glob pattern") {
    //     match entry {
    //         Ok(path) => println!("{:?}", path.display()),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }

    //  for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    //     println!("{}", e.path().display());
    // }
    // // 
    //     for entry in fs::read_dir(file_pattern).expect("hello") {
    //         let entry = entry.expect("Failed dir");
    //         let path = entry.path();
    //         if path.is_file() {
    //             println!("{}", path.display());
    //         } 
    //     }
    // } 


    let files = get_files(file_pattern);

    for f in files.iter() {
        println!("{:?}", f);
    }

    let regex = Regex::new(regex_pattern).expect("Failed to compile file regex");
    for f in files {
        let original_name = String::from( f.file_name().to_str().unwrap() );
        let new_name = regex.replace_all(&original_name, regex_replace_pattern);
        println!("{:?} --> {}", original_name, new_name);
    }
}


fn get_files(file_pattern: &str) -> Vec<DirEntry> {
    let mut files = Vec::new();
    let glob = Glob::new(file_pattern).expect("Wrong glob file pattern").compile_matcher();

    for entry in fs::read_dir(".").expect("hello") {
        let entry = entry.expect("Failed dir");
        let path = entry.path();
        if path.is_file() && glob.is_match(path) {
            files.push(entry);
        }
    }

    return  files;
}