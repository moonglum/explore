use std::fs::{DirEntry, FileType};
use std::os::unix::fs::{FileTypeExt};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, TimeZone, Local};
use chrono::format::{DelayedFormat, StrftimeItems};
use std::os::unix::fs::{PermissionsExt};

pub struct Entry {
    pub file_type: FileType,
    pub permissions: u32,
    pub modified: SystemTime,
    pub file_name: String,
    pub size: u64
}

fn system_time_to_date_time(system_time: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match system_time.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(_) => panic!("An Error Occurred!"),
    };
    Local.timestamp(sec, nsec)
}

impl Entry {
    pub fn from_dir_entry(entry: &DirEntry) -> Entry {
        let file_name = entry.file_name().into_string().expect("File name could not be determined");
        let metadata = entry.metadata().expect("Meta Data could not be determined");
        let modified = metadata.modified().expect("Modified Date could not be determined");
        let size = metadata.len();

        Entry {
            file_type: metadata.file_type(),
            permissions: metadata.permissions().mode(),
            modified: modified,
            file_name: file_name,
            size: size
        }
    }

    pub fn to_s(&self) -> String {
        format!("{}{} {:7} {} {}",
            self.file_type(),
            self.permissions(),
            self.size,
            self.modified(),
            self.file_name
        )
    }

    fn modified(&self) -> DelayedFormat<StrftimeItems> {
        system_time_to_date_time(self.modified).format("%b %d %R")
    }

    fn file_type(&self) -> char {
        if self.file_type.is_file() {
            '-'
        } else if self.file_type.is_dir() {
            'd'
        } else if self.file_type.is_symlink() {
            'l'
        } else if self.file_type.is_fifo() {
            'p'
        } else if self.file_type.is_socket() {
            's'
        } else if self.file_type.is_char_device() {
            'c'
        } else if self.file_type.is_block_device() {
            'b'
        } else {
            '?'
        }
    }

    fn permissions(&self) -> String {
        let mut string_notation = String::from("");

        string_notation.push(self.convert_permission_bit(0, 'r'));
        string_notation.push(self.convert_permission_bit(1, 'w'));
        string_notation.push(self.convert_permission_bit(2, 'x'));
        string_notation.push(self.convert_permission_bit(3, 'r'));
        string_notation.push(self.convert_permission_bit(4, 'w'));
        string_notation.push(self.convert_permission_bit(5, 'x'));
        string_notation.push(self.convert_permission_bit(6, 'r'));
        string_notation.push(self.convert_permission_bit(7, 'w'));
        string_notation.push(self.convert_permission_bit(8, 'x'));

        string_notation
    }

    fn convert_permission_bit(&self, position: u32, permission_type: char) -> char {
        match (self.permissions / (2u32.pow(8 - position))) % 2 == 1 {
            true => permission_type,
            false => '-',
        }
    }
}
