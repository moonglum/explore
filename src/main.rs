extern crate chrono;
mod entry;
use std::fs::{DirEntry, ReadDir, read_dir};
use entry::Entry;
use std::env;

fn main() {
    let dir = read_dir(determine_target()).expect("Directory could not be read");
    iterate_directory(dir, &Entry::from_dir_entry)
}

fn determine_target() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
      format!("{}", "./")
    } else {
      format!("{}", args[1])
    }
}

fn iterate_directory(dir: ReadDir, cb: &Fn(&DirEntry) -> Entry) {
    let paths = dir.map(|path| cb(&path.expect("Entry could not be read")));

    for path in paths {
        println!("{}", path.to_s());
    }
}
