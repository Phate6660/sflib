use std::error::Error as Error;
use std::fs;
use std::io::Read;

/// This function takes:
/// - The path of a dir as an `str`.
/// - Whether or not parents should be made as a `bool`.
/// And returns an error if the dir(s) cannot be created.
pub fn ensure_directory(path: &str, make_parents: bool) -> Result<(), Box<dyn Error>> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        if make_parents {
            fs::create_dir_all(path)?;
        } else {
            fs::create_dir(path)?;
        }
    }
    Ok(())
}

/// This function will read the contents of a file and return it as a `String`.
pub fn read(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = std::fs::File::open(file_path)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// This function return the specified line of a file as a `String`.
pub fn line(file: &str, line: usize) -> Result<String, Box<dyn Error>> {
    let contents = read(file)?;
    Ok(contents.split('\n').collect::<Vec<&str>>()[line].to_string())
}
