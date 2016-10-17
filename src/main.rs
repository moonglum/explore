extern crate chrono;
use std::io;
use std::fs::{self, DirEntry, FileType};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::FileTypeExt;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, TimeZone, Local};

fn main() {
    match iterate_directory("./", &print_entry) {
        Ok(_) => println!("Everything went well"),
        Err(e) => println!("An Error Occurred! {}", e),
    }
}

fn iterate_directory(foo: &str, cb: &Fn(&DirEntry)) -> io::Result<()> {
    let paths = try!(fs::read_dir(foo));

    for path in paths {
        let entry = try!(path);
        cb(&entry);
    }

    Ok(())
}

fn print_entry(entry: &DirEntry) {
    match entry.file_name().into_string() {
        Ok(file_name) => {
            match entry.metadata() {
                Ok(metadata) => {
                    match metadata.modified() {
                        Ok(modified) => {
                            let datetime = system_time_to_date_time(modified);

                            println!("{}{} {} {}", convert_file_type(
                                metadata.file_type()),
                                convert_permissions(metadata.permissions().mode()),
                                datetime.format("%b %d %R"),
                                file_name
                            )
                        },
                        Err(_) => println!("An Error Occurred!"),
                    }
                },
                Err(_) => println!("An Error Occurred!"),
            }
        },
        Err(_) => println!("An Error Occurred!"),
    }
}

fn convert_file_type(file_type: FileType) -> char {
    if file_type.is_file() {
        '-'
    } else if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else {
        '?'
    }
}

fn convert_permission_bit(numeric_notation: u32, position: u32, permission_type: char) -> char {
    match (numeric_notation / (2u32.pow(8 - position))) % 2 == 1 {
        true => permission_type,
        false => '-',
    }
}

fn convert_permissions(numeric_notation: u32) -> String {
    let mut string_notation = String::from("");

    string_notation.push(convert_permission_bit(numeric_notation, 0, 'r'));
    string_notation.push(convert_permission_bit(numeric_notation, 1, 'w'));
    string_notation.push(convert_permission_bit(numeric_notation, 2, 'x'));
    string_notation.push(convert_permission_bit(numeric_notation, 3, 'r'));
    string_notation.push(convert_permission_bit(numeric_notation, 4, 'w'));
    string_notation.push(convert_permission_bit(numeric_notation, 5, 'x'));
    string_notation.push(convert_permission_bit(numeric_notation, 6, 'r'));
    string_notation.push(convert_permission_bit(numeric_notation, 7, 'w'));
    string_notation.push(convert_permission_bit(numeric_notation, 8, 'x'));

    string_notation
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Local.timestamp(sec, nsec)
}
