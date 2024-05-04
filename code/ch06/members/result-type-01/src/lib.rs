use std::fs;
use std::io;
use std::io::prelude::*;

use std::env;

#[derive(Debug)]
pub enum CustomError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
}
impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::IoError(error)
    }
}
impl From<std::num::ParseIntError> for CustomError {
    fn from(error: std::num::ParseIntError) -> Self {
        CustomError::ParseError(error)
    }
}

pub fn create_file_path(subdirectories: Vec<&str>, file_name: &str) -> Option<String> {
    if let Ok(mut current_dir) = env::current_dir() {
        for subdir in subdirectories {
            current_dir.push(subdir);
        }
        current_dir.push(file_name);
        let path = current_dir.to_str().unwrap();
        Some(path.to_string())
    } else {
        None
    }
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    // Open the file.
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Current dir = {}",
                std::env::current_dir().unwrap().to_str().unwrap()
            );
            return Err(e);
        }
    };

    // Read the contents of the file.
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => {
            eprint!(
                "Current dir = {}",
                std::env::current_dir().unwrap().to_str().unwrap()
            );
            Err(e)
        }
    }
}

pub fn read_file2(filename: &str) -> Result<i32, CustomError> {
    // Open the file.
    let mut file = fs::File::open(filename)?;

    // Read the contents of the file.
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(CustomError::from)?;
   
   // Parse the contents of the file. 
   let value: i32 = contents.trim().parse().map_err(CustomError::from)?; 
   Ok(value)
}
