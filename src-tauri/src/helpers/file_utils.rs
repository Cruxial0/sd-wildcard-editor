use std::{fs::File, io::{BufRead, BufReader}, path::{self, Path, PathBuf}};

use super::dir_utils::get_public_directory;

pub fn from_relative(path: impl AsRef<Path>) -> PathBuf {
    Path::new(&get_public_directory()).join(path)
}

// Sourced from: https://stackoverflow.com/a/35820003
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(from_relative(filename)).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

