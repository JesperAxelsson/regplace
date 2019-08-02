use std::io;
use std::fs::{self, DirEntry, rename};
//use std::path::Path;
// 
use std::path::PathBuf;

use structopt::StructOpt;
use globset::Glob;
use globset::GlobMatcher;
use regex::Regex;

#[derive(StructOpt, Debug)]
#[structopt(name = "Regex replace", author = "Jesper Axelsson <jesperaxe@gmail.com>", about="Rename files with regex")]
struct Opt {
    #[structopt(required=true, index=1, help="Sets regex capture pattern")]
    regex_pattern: String,

    #[structopt(required=true, index=2, help="Sets the regex replace pattern")]
    regex_replace_pattern: String,

    #[structopt(short, long, default_value=".", parse(from_os_str))]
    path: PathBuf,
    
    #[structopt(short, long, default_value="*", help="glob file pattern")]
    file_pattern: String,

    #[structopt(short, long, help="Overwrite existing files")]
    overwrite: bool,

    #[structopt(short, long, help="Go into sub directories")]
    recursive: bool,

    #[structopt(short, long, help="Perform a dry tun to see changes")]
    dry_run:bool, 
    
    #[structopt(short, long, help="Sets the level of verbosity")]
    verbose:bool, 
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    if opt.verbose {
        println!("{:#?}", opt);
    }

    let glob = Glob::new(&opt.file_pattern).expect("Wrong glob file pattern").compile_matcher();
    let mut files = Vec::new();

    get_files(opt.recursive, &mut files, &opt.path, &glob);

    if opt.verbose {
        for f in files.iter() {
                println!("{:?}", f);
        }
    }

    let regex = Regex::new(&opt.regex_pattern).expect("Failed to compile file regex");

    for f in files {
        let original_name = String::from(f.file_name().to_str().unwrap());
        
        if regex.is_match(&original_name) {
            let rpl: &str = &opt.regex_replace_pattern;
            let new_name = regex.replace_all(&original_name, rpl);
            let mut new_path = f.path().clone();
            new_path.set_file_name(new_name.into_owned());
            
            if !new_path.exists() {
                println!("'{}' --> '{}'", f.path().display(), new_path.display());
            } else {
                println!("'{}' --> '{}' already exists", f.path().display(), new_path.display());
            }

            if !opt.dry_run && (!new_path.exists() || opt.overwrite) {
                // Change the names!
                rename(f.path(), new_path)?;
            }
        }
    }

    Ok(())
}


fn get_files(recursive: bool, files: &mut Vec<DirEntry>, path: &PathBuf, glob: &GlobMatcher) {
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed dir");
        let path = entry.path();
        if path.is_file() && glob.is_match(&path) {
            files.push(entry);
        } else if path.is_dir() && recursive {
            get_files(recursive, files, &path, glob);
        }        
    }
}
