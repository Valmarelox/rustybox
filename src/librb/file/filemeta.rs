use chrono::DateTime;
use chrono::Local;
use core::convert::TryFrom;
use core::fmt;
use core::fmt::Formatter;
use core::option::Option::{None, Some};
use core::option::Option;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use core::result::Result::Ok;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use chrono::Duration;
use users::get_user_by_uid;
use crate::librb::file::filetype::FileType;
use crate::librb::file::permissions::{PermissionsMask};

pub struct FileMetadata {
    name: String,
    permissions: PermissionsMask,
    size: u64,
    file_type: FileType,
    mtime: SystemTime,
    uid: u32
}

impl FileMetadata {
    pub fn is_hidden(&self) -> bool {
        self.name.starts_with(".")
    }
}

impl fmt::Display for FileMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_type)?;
        write!(f, "{} ", self.permissions)?;
        write!(f, "{:<8} ", self.size)?;

        if let Some(user) = get_user_by_uid(self.uid) {
            if let Some(username) = user.name().to_str() {
                write!(f, "{:<8}", username)?;
            } else {
                write!(f, "{:<8}", self.uid)?;
            }
        } else {
            write!(f, "{:<8}", self.uid)?;
        }
        let date: DateTime<Local> = self.mtime.into();
        // TODO: Move to a seperate type
        if (date - Local::now()) > Duration::seconds( 24 * 60 * 60 ) {
            write!(f, "{:<8} ", date.format("%d %b %Y"))?;
        } else {
            write!(f, "{:<8} ", date.format("%d %b %H:%M"))?;
        }
        write!(f, "{:<16}", self.name)
    }
}

pub fn get_meta(p: &Path) -> Option<FileMetadata> {
    if let Ok(f) = fs::symlink_metadata(p) {
        let name = p.file_name().unwrap().to_str().unwrap().to_string();
        let size = f.len();
        let uid = f.uid();
        let mode = f.permissions().mode();
        if let Ok(mtime) = f.modified() {
            if let Ok(file_type) = FileType::try_from(f) {
                return Some(FileMetadata {
                    name,
                    permissions: PermissionsMask::build(mode),
                    size,
                    mtime,
                    uid,
                    file_type,
                });
            }
        }
    }
    return None;
}
#[cfg(test)]
mod tests {
    use std::io;
    use std::path::PathBuf;
    use std::process::Command;

    use users::get_current_uid;
    use super::FileType;
    use super::get_meta;
    use super::PermissionsMask;


    #[test]
    fn test_current_file_metadata() -> Result<(), io::Error>{
        Command::new("sh")
            .arg("-c")
            .arg("truncate -s4 /tmp/a; chmod 0644 /tmp/a")
            .output()
            .expect("failed to execute process");
        let good_uid: u32 = get_current_uid();
        let meta = get_meta(&PathBuf::from("/tmp/a"));
        match meta {
            Some(meta) => {
                assert_eq!(meta.file_type, FileType::RegularFile);
                // TODO: What is the highest bit?
                // TODO: also this is bad that this is hardcoded
                assert_eq!(meta.permissions,
                           PermissionsMask::S_IRUSR | PermissionsMask::S_IWUSR |
                               PermissionsMask::S_IRGRP | PermissionsMask::S_IROTH
                );
                assert_eq!(meta.uid, good_uid);
                assert_eq!(meta.size, 4);
                assert_eq!(meta.name, "a");
            }
            None => assert!(false)
        }
        Ok(())
    }
}

