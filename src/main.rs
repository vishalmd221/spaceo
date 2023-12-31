// #![allow(unused)] // Silence unused warning while exploring (to comment out)

mod argc;
mod error;

use argc::arc_app;
use error::AppError;
use clap::ArgMatches;
use file_size::fit_4;
use walkdir::WalkDir;
use std::{ error::Error, path::PathBuf };

const TOP_NUMS: usize = 5;
const DIR: &str = "./";

fn main() {
    let argc = arc_app().get_matches();

    let options = match Options::from_argc(argc) {
        Ok(options) => options,
        Err(ex) => {
            println!("Error passing input {}", ex );
            return;
        }
    };

    match exec(options) {
        Ok(_) => (),
        Err(ex) => {
            println!("ERROR - {}", ex);
        }
    }
}

struct Entry {
    path: PathBuf,
    size: u64,
}

struct Options {
    nums: usize,
}

impl Options {
    fn from_argc(argc: ArgMatches) -> Result<Options, AppError> {
        let nums: usize = match argc
            .value_of("nums"){
                None => TOP_NUMS,
                Some(nums) => {
                    nums.parse::<usize>().or_else(|_| Err(AppError::InvalidNumberOfFiles(nums.to_string())))?
                }
            };
            
        Ok(Options { nums })
    }
}

fn exec(options: Options) -> Result<(), Box<dyn Error>> {
    let mut total_size: u64 = 0;
    let mut total_numbers: u32 = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(options.nums + 1);
    let mut min_of_tops = 0;

    for entry in WalkDir::new(DIR) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && !entry.path_is_symlink() {
            total_numbers += 1;
            let size = entry.metadata()?.len();
            total_size += size;

            if min_of_tops < size {
                tops.push(Entry {
                    path: entry.path().to_path_buf(),
                    size,
                });
                tops.sort_by(|a, b| b.size.cmp(&a.size));
                if tops.len() > options.nums {
                    tops.pop();
                }
                min_of_tops = tops
                    .last()
                    .map(|e| e.size)
                    .unwrap_or(0);
            }
        }
        // println!("{}", entry.path().display());
    }
    println!("Number of files: {}, total_size: {}", total_numbers, fit_4(total_size));
    println!("Top biggest files: {}", tops.len());
    for Entry { size, path } in tops.iter() {
        println!("{:<4} - {}", fit_4(*size), path.to_string_lossy());
    }
    Ok(())
}
