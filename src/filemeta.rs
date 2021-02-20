use std::{fmt, fs};
use std::fmt::Formatter;
use std::path::Path;
use std::time::SystemTime;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
//use std::os::linux::fs::PermissionsExt;

use chrono::{DateTime, Duration, Local};
use strum_macros::EnumString;
use users::get_user_by_uid;
use std::fs::Metadata;
use std::convert::TryFrom;
use num_enum::TryFromPrimitiveError;


bitflags! {
    struct FilePermissions: u32 {
        const PF_R = 0o4;
        const PF_W = 0o2;
        const PF_X = 0o1;
    }
}

bitflags! {
    struct PermissionsMask: u32 {
        const S_IRUSR = 0o0400;
        const S_IWUSR = 0o0200;
        const S_IXUSR = 0o0100;
        const S_IRGRP = 0o0040;
        const S_IWGRP = 0o0020;
        const S_IXGRP = 0o0010;
        const S_IROTH = 0o0004;
        const S_IWOTH = 0o0002;
        const S_IXOTH = 0o0001;
    }

}

impl PermissionsMask {
    pub fn user(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 6) & 0o7}
    }
    pub fn group(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 3) & 0o7}
    }
    pub fn other(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 0) & 0o7}
    }
}

impl fmt::Display for FilePermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contains(FilePermissions::PF_R) {
            write!(f, "r")?;
        } else {
            write!(f, "-")?;
        }
        if self.contains(FilePermissions::PF_W) {
            write!(f, "w")?;
        } else {
            write!(f, "-")?;
        }
        if self.contains(FilePermissions::PF_X) {
            write!(f, "x")?;
        } else {
            write!(f, "-")?;
        }
    Ok(())
    }
}

impl fmt::Display for PermissionsMask{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user())?;
        write!(f, "{}", self.group())?;
        write!(f, "{}", self.other())
    }
}

#[derive(PartialEq, Display, Debug, EnumString,TryFromPrimitive)]
#[repr(u32)]
enum FileType {
    #[strum(serialize="s")]
    Socket          = 0o140000,
    #[strum(serialize="l")]
    SymbolicLink    = 0o120000,
    #[strum(serialize=".")]
    RegularFile     = 0o100000,
    #[strum(serialize="b")]
    BlockDevice     = 0o060000,
    #[strum(serialize="d")]
    Directory       = 0o040000,
    #[strum(serialize="c")]
    CharDevice      = 0o020000 ,
    #[strum(serialize="f")]
    Fifo            = 0o010000,
}

impl TryFrom<Metadata> for FileType {
    type Error = TryFromPrimitiveError<Self>;

    fn try_from(f: Metadata) -> Result<Self, Self::Error> {
        FileType::try_from(f.mode() & 0o170000)
    }
}

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
                    permissions: PermissionsMask { bits: mode & 0o777 },
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
    use std::process::Command;

    use users::get_current_uid;

    use crate::filemeta::{get_meta, FileType};
    use std::path::PathBuf;
    use std::io;

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
                assert_eq!(meta.permissions.bits, 0o644);
                assert_eq!(meta.uid, good_uid);
                assert_eq!(meta.size, 4);
                assert_eq!(meta.name, "a");
            }
            None => assert!(false)
        }
        Ok(())
    }
}
