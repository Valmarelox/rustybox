use std::{fs, fmt};
use std::env;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::fmt::{Error, Debug, Formatter};
use std::io;
use std::fs::Permissions;
use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::{DateTime, Local, Duration};
use users::get_user_by_uid;

#[macro_use]
extern crate bitflags;
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
    fn user(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 6) & 0o7}
    }
    fn group(&self) -> FilePermissions {
        FilePermissions{bits: (self.bits >> 3) & 0o7}
    }
    fn other(&self) -> FilePermissions {
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
        write!(f, "{}", self.user());
        write!(f, "{}", self.group());
        write!(f, "{}", self.other())
    }
}

struct FileMetadata {
    permissions: PermissionsMask,
    size: u64,
    is_dir: bool,
    mtime: SystemTime,
    uid: u32
}

impl fmt::Display for FileMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", if self.is_dir { 'd' } else { '.' })?;
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
            write!(f, "{:<8} ", date.format("%d %b %Y"))
        } else {
            write!(f, "{:<8} ", date.format("%d %b %H:%M"))
        }
    }
}

fn get_meta(p: &Path) -> Option<FileMetadata> {
    if let Ok(f) = fs::metadata(p) {
        let size = f.len();
        let uid = f.uid();
        if let Ok(mtime) = f.modified() {
            if let mode = f.permissions().mode() {
                return Some(FileMetadata {
                    permissions: PermissionsMask { bits: mode },
                    size,
                    mtime,
                    uid,
                    is_dir: f.is_dir(),
                });
            }
        }
    }
    return None;
}

fn list_dirs() -> Result<String, io::Error>{
    let path = std::env::current_dir()?;
    for entry in std::fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if let Some(x) = entry.file_name().to_str() {
                if let Some(meta) = get_meta(&entry.path()) {
                    println!("{} {}", meta, x)
                }
            }
        }
    }
    Ok(("S".to_string()))
}


fn main() {
    list_dirs();
    println!("Hello, world!");
}
