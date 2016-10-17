extern crate chrono;
mod entry;
use std::fs::{DirEntry, ReadDir, read_dir};
use entry::Entry;

fn main() {
    let directory_name = "./";
    let dir = read_dir(directory_name).expect("Directory could not be read");
    iterate_directory(dir, &Entry::from_dir_entry)
}

fn iterate_directory(dir: ReadDir, cb: &Fn(&DirEntry) -> Entry) {
    let paths = dir.map(|path| cb(&path.expect("Entry could not be read")));

    for path in paths {
        println!("{}", path.to_s());
    }
}
