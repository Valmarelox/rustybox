use std::{fs, fmt};

use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::fmt::Formatter;
use std::io;
use std::time::SystemTime;
extern crate chrono;
use chrono::{DateTime, Local, Duration};
use users::get_user_by_uid;
use clap::{Arg, App, ArgMatches, Values};

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
        write!(f, "{}", self.user())?;
        write!(f, "{}", self.group())?;
        write!(f, "{}", self.other())
    }
}

struct FileMetadata {
    name: String,
    permissions: PermissionsMask,
    size: u64,
    is_dir: bool,
    mtime: SystemTime,
    uid: u32
}

impl FileMetadata {
    fn is_hidden(&self) -> bool {
        self.name.starts_with(".")
    }
}

impl fmt::Display for FileMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.is_dir { 'd' } else { '.' })?;
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

fn get_meta(p: &Path) -> Option<FileMetadata> {
    if let Ok(f) = fs::metadata(p) {
        let name = p.file_name().unwrap().to_str().unwrap().to_string();
        let size = f.len();
        let uid = f.uid();
        let mode = f.permissions().mode();
        if let Ok(mtime) = f.modified() {
            return Some(FileMetadata {
                name,
                permissions: PermissionsMask { bits: mode },
                size,
                mtime,
                uid,
                is_dir: f.is_dir(),
            });
        }
    }
    return None;
}

struct DisplayFormat {
    show_hidden: bool,
    color: String
}

impl DisplayFormat {
    fn should_diplay(&self, f: &FileMetadata) -> bool {
        (!f.is_hidden() || self.show_hidden)
    }
}

fn list_dirs(path: &PathBuf, fmt: &DisplayFormat) -> Result<(), io::Error>{
    for entry in std::fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if let Some(meta) = get_meta(&entry.path()) {
                if fmt.should_diplay(&meta) {
                    println!("{}", meta)
                }
            }
        }
    }
    Ok(())
}

fn build_display_fmt(matches: &ArgMatches) -> DisplayFormat {
    DisplayFormat {
        show_hidden: matches.is_present("all"),
        color: matches.value_of("color").unwrap_or("auto").to_string()
    }
}

fn print_dirs(dirs: Option<Values>, fmt: &DisplayFormat) -> Result<(), std::io::Error> {
    match dirs {
        Some(dirs) => {
            for dir in dirs {
                let path = PathBuf::from(dir);
                list_dirs(&path, &fmt)?;
                println!("");
            }
        }
        None => {
            list_dirs(&std::env::current_dir()?, &fmt)?;
        }
    }
    Ok(())
}

fn get_arguments() -> ArgMatches<'static>  {
    App::new("rustybox")
        .version("0.0.1")
        .author("Efi Weiss <valmarelox@gmail.com>")
        .about("A not that busy (yet!) and still a bit rusty box")
        .arg(
            Arg::with_name("all").short("-a").long("-all").takes_value(false).help("show hidden and 'dot' files")
        ).arg(
            Arg::with_name("color").short("-c").long("-color").takes_value(true).possible_values(&["never", "auto", "always"]).help("show hidden and 'dot' files")
        ).arg(
            Arg::with_name("directories").multiple(true)
    ).get_matches()
}

fn main() -> Result<(), std::io::Error> {
    let matches = get_arguments();
    let fmt = build_display_fmt(&matches);
    let dirs = matches.values_of("directories");
    print_dirs(dirs, &fmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use users::get_current_uid;

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
                assert_eq!(meta.is_dir, false);
                // TODO: What is the highest bit?
                // TODO: also this is bad that this is hardcoded
                assert_eq!(meta.permissions.bits, 0o100644);
                assert_eq!(meta.uid, good_uid);
                assert_eq!(meta.size, 4);
                assert_eq!(meta.name, "a");
            }
            None => assert!(false)
        }
        Ok(())
    }
}