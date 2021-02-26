use clap::{App, Arg, ArgMatches, Values, SubCommand};
use core::option::Option::{None, Some};
use core::option::Option;
use std::os::unix::fs::DirEntryExt;
use core::result::Result;
use core::result::Result::Ok;
use std::io::Write as IoWrite;
use std::fmt::Write as FmtWrite;
use std::io;
use std::path::PathBuf;
use crate::librb::file::filemeta::{FileMetadata};
use std::str::FromStr;
use strum_macros::EnumString;

pub fn subcommand() -> App<'static, 'static>  {
    SubCommand::with_name("ls")
        .about("List files")
        .arg(
            Arg::with_name("all").short("-a").long("-all").takes_value(false).help("show hidden and 'dot' files")
        ).arg(
            Arg::with_name("color").short("-c").long("-color").takes_value(true).possible_values(&["never", "auto", "always"]).help("Color the output")
        ).arg(
            Arg::with_name("long-display").short("-l").takes_value(false).help("use long listing format")
        ).arg(
            Arg::with_name("directories").help("Files/Directories to list").multiple(true).index(1)
    )
}

fn display_entry(meta: FileMetadata, fmt: &DisplayFormat, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    if fmt.long_display {
        return writeln!(*writer, "{}", meta);
    }

    write!(*writer, "{} ", meta.short_name())
}

fn list_dirs(path: &PathBuf, fmt: &DisplayFormat, writer: &mut impl std::io::Write) -> Result<(), io::Error>{
    /*if fmt.show_hidden {
        let meta = FileMetadata::for_path(&path.join(".")).unwrap();
        if fmt.should_diplay(&meta) {
            display_entry(meta, fmt, writer)?;
        }
        let meta = FileMetadata::for_path(&path.join("..")).unwrap();
        if fmt.should_diplay(&meta) {
            display_entry(meta, fmt, writer)?;
        }
    }*/
    for entry in std::fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if let Some(meta) = FileMetadata::for_path(&entry.path()) {
                if fmt.should_diplay(&meta) {
                    display_entry(meta, fmt, writer)?;
                }
            }
        }
    }
    Ok(())
}

fn print_dirs(dirs: Option<Values>, fmt: &DisplayFormat, writer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
    match dirs {
        Some(dirs) => {
            for dir in dirs {
                let path = PathBuf::from(dir);
                list_dirs(&path, &fmt, writer)?;
            }
        }
        None => {
            list_dirs(&std::env::current_dir()?, &fmt, writer)?;
        }
    }
    Ok(())
}

fn _ls_main(matches: Option<&ArgMatches>, writer: &mut impl std::io::Write) -> Result<(), String> {
    let matches = matches.ok_or("wtf")?;
    let fmt = build_display_fmt(matches);
    let dirs = matches.values_of("directories");
    print_dirs(dirs, &fmt, writer).or(Err("print failed".to_string()))
}

pub fn ls_main(matches: Option<&ArgMatches>) -> Result<(), String> {
    _ls_main(matches, &mut std::io::stdout())
}


#[derive(EnumString, PartialEq)]
enum ColorOption {
    #[strum(serialize = "never")]
    Never,
    #[strum(serialize = "auto")]
    Auto,
    #[strum(serialize = "always")]
    Always
}

struct DisplayFormat {
    show_hidden: bool,
    long_display: bool,
    color: ColorOption
}

impl DisplayFormat {
    fn should_diplay(&self, f: &FileMetadata) -> bool {
        !f.is_hidden() || self.show_hidden
    }
}

fn build_display_fmt(matches: &ArgMatches) -> DisplayFormat {
    DisplayFormat {
        show_hidden: matches.is_present("all"),
        long_display: matches.is_present("long-display"),
        color: ColorOption::from_str(&matches.value_of("color").unwrap_or("auto").to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::librb::file::filemeta::tests::setup_test;
    use std::process::{Command, Output};
    use super::{_ls_main, subcommand, build_display_fmt};
    use std::ffi::OsStr;
    use std::str;
    use users::{get_user_by_uid, get_current_uid};
    use crate::applets::ls::ColorOption;

    #[test]
    fn test_arg_parse() {
        // ls -a -l /tmp/aaa
        let args: [&OsStr; 4] = [OsStr::new("ls"), OsStr::new("-a"), OsStr::new("-l"), OsStr::new("/tmp/aaa")];
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        let fmt = build_display_fmt(&matches);
        assert!(fmt.show_hidden);
        assert!(fmt.long_display);
        assert!(fmt.color == ColorOption::Auto);

        // ls /tmp/aaa
        let args: [&OsStr; 2] = [OsStr::new("ls"), OsStr::new("/tmp/aaa")];
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        let fmt = build_display_fmt(&matches);
        assert!(!fmt.show_hidden);
        assert!(!fmt.long_display);
        assert!(fmt.color == ColorOption::Auto);
    }

    fn run_cmd(cmd: &str) -> Output {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    }

    #[test]
    fn test_print_dir() {
        // TODO: Create dir with files and check outputs
        // TODO: Create dir with files and check hidden outputs
        let dir = "/tmp/rustybox-test/ccc";
        run_cmd(&format!("rm -rf {dir}; mkdir -p {dir}; touch {dir}/a {dir}/b", dir=dir));
        let args: [&OsStr; 2] = [OsStr::new("ls"), OsStr::new(dir)];
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        let mut output : Vec<u8> = Vec::new();
        assert!(_ls_main(Some(&matches), &mut output).is_ok());
        println!("wtf {:?}", output);
        assert_eq!(str::from_utf8(&output).unwrap(), "b a ");
    }
    #[test]
    fn test_print_dir_hidden_file() {
        // TODO: Create dir with files and check outputs
        // TODO: Create dir with files and check hidden outputs
        let dir = "/tmp/rustybox-test/ccc";
        run_cmd(&format!("rm -rf {dir}; mkdir -p {dir}; touch {dir}/a {dir}/b {dir}/.c", dir=dir));
        let args: [&OsStr; 3] = [OsStr::new("ls"), OsStr::new("-a"), OsStr::new(dir)];
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        let mut output : Vec<u8> = Vec::new();
        assert!(_ls_main(Some(&matches), &mut output).is_ok());
        println!("wtf {:?}", output);
        assert_eq!(str::from_utf8(&output).unwrap(), ".c b a ");
    }
}