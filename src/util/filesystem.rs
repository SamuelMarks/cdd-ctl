use crate::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn file_search(filename: &str, pwd: bool, home: bool) -> Option<String> {
    let mut paths: Vec<PathBuf> = vec![];

    if pwd {
        paths.push(PathBuf::from(format!("./{}", filename)));
    }
    if home {
        if let Ok(home_dir) = home_dir() {
            paths.push(home_dir);
        }
    }

    for path in paths {
        if let Ok(file) = read_file(path) {
            return Some(file);
        }
    }

    None
}

pub fn file_exists<S: std::convert::AsRef<std::ffi::OsStr>>(filename: S) -> bool {
    Path::new(&filename).exists()
}

pub fn read_file(pathbuf: PathBuf) -> CliResult<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn write_file(pathbuf: PathBuf, content: &str) -> Result<(), failure::Error> {
    Ok(std::fs::write(pathbuf, content)?)
}

pub fn copy_file<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> CliResult<()> {
    Ok(fs::copy(from, to).map(|_| ())?)
}

pub fn copy_dir<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> CliResult<()> {
    use fs_extra::dir::*;
    let options = CopyOptions::new();
    Ok(copy(from, to, &options).map(|_| ())?)
}

// pub fn copy_files_recursively(pathbuf: PathBuf) -> CliResult<()> {
//     Ok(())
// }

pub fn home_dir() -> CliResult<PathBuf> {
    Ok(dirs::home_dir().ok_or(failure::format_err!(
        "There was a problem locating your home directory."
    ))?)
}

pub fn expand_home_path(path: String) -> CliResult<String> {
    let home_dir: String = home_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| failure::format_err!("Could not convert OS string to rust string."))?;
    Ok(path.replace("~", &home_dir))
}

#[test]
fn test_find_file() {
    assert_eq!(file_search("fakefile", true, false), None);
    assert_eq!(file_search("Cargo.toml", true, false).is_some(), true);
    assert_eq!(file_search("Cargo.toml", false, false), None);
}
