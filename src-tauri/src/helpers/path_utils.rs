use std::path::PathBuf;

pub fn pathbuf_filename(pathbuf: &PathBuf) -> String {
    pathbuf.file_name().unwrap().to_str().unwrap().to_owned()
}