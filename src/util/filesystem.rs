use crate::*;
use std::fs;
use std::path::{Path, PathBuf};

// pub fn home_dir() -> CliResult<String> {
//     Ok(dirs::home_dir()
//         .ok_or(|_| failure::format_err!("config.yml already exists."))?
//         .into_os_string()
//         .into_string()
//         .map_err(|_| failure::format_err!("config.yml already exists."))?)
//     dirs::home_dir()
// }

pub fn file_search(filename: &str, pwd: bool, home: bool) -> Option<String> {
    let mut paths: Vec<PathBuf> = vec![];

    if pwd {
        paths.push(PathBuf::from(format!("./{}", filename)));
    }
    if home {
        paths.push(PathBuf::from(format!("~/{}", filename)));
    }

    for path in paths {
        if let Ok(file) = read_file(path) {
            return Some(file);
        }
    }

    None
}

pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn read_file(pathbuf: PathBuf) -> CliResult<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn copy_file<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> CliResult<()> {
    Ok(fs::copy(from, to).map(|_| ())?)
}

pub fn copy_files_recursively(pathbuf: PathBuf) -> CliResult<()> {
    Ok(())
}

#[test]
fn test_find_file() {
    assert_eq!(file_search("fakefile", true, false), None);
    assert_eq!(file_search("Cargo.toml", true, false).is_some(), true);
    assert_eq!(file_search("Cargo.toml", false, false), None);
}
