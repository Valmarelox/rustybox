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
    pub fn for_path(p: &Path) -> Option<FileMetadata> {
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

#[cfg(test)]
pub mod tests {
    use std::io;
    use std::path::PathBuf;
    use std::process::{Command, Output};

    use users::get_current_uid;
    use super::FileType;
    use super::PermissionsMask;
    use super::FileMetadata;

    pub struct TestCaseData {
        path: String,
        name: String,
        size: u64,
        permissions: PermissionsMask,
        uid: u32
    }

    fn create_file(data: &TestCaseData) -> Output {
        Command::new("sh")
            .arg("-c")
            .arg(format!("truncate -s{size} {name}; chmod {permissions} {name}", size=data.size, name=data.path, permissions="0644"))
            .output()
            .expect("failed to execute process")
    }

    pub fn setup_test() -> TestCaseData {
        let case = TestCaseData {
            path: "/tmp/a".to_string(),
            name: "a".to_string(),
            size: 4,
            permissions: PermissionsMask::S_IRUSR | PermissionsMask::S_IWUSR |
                PermissionsMask::S_IRGRP | PermissionsMask::S_IROTH,
            uid: get_current_uid()
        };
        create_file(&case);
        case
    }

    #[test]
    fn test_current_file_metadata() -> Result<(), io::Error>{
        let case = setup_test();
        let meta = FileMetadata::for_path(&PathBuf::from(case.path));
        match meta {
            Some(meta) => {
                assert_eq!(meta.file_type, FileType::RegularFile);
                // TODO: What is the highest bit?
                // TODO: also this is bad that this is hardcoded
                assert_eq!(meta.permissions,
                           PermissionsMask::S_IRUSR | PermissionsMask::S_IWUSR |
                               PermissionsMask::S_IRGRP | PermissionsMask::S_IROTH
                );
                assert_eq!(meta.uid, case.uid);
                assert_eq!(meta.size, case.size);
                assert_eq!(meta.name, case.name);
            }
            None => assert!(false)
        }
        Ok(())
    }
}

