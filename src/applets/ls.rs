use clap::{App, Arg, ArgMatches, Values};
use core::option::Option::{None, Some};
use core::option::Option;
use core::result::Result;
use core::result::Result::Ok;
use std::io;
use std::path::PathBuf;
use crate::librb::file::filemeta::{FileMetadata};
use std::str::FromStr;
use strum_macros::EnumString;

fn get_arguments() -> ArgMatches<'static>  {
    App::new("rustybox")
        .version("0.0.1")
        .author("efi weiss <valmarelox@gmail.com>")
        .about("a not that busy (yet!) and still a bit rusty box")
        .arg(
            Arg::with_name("all").short("-a").long("-all").takes_value(false).help("show hidden and 'dot' files")
        ).arg(
            Arg::with_name("color").short("-c").long("-color").takes_value(true).possible_values(&["never", "auto", "always"]).help("show hidden and 'dot' files")
        ).arg(
            Arg::with_name("directories").multiple(true)
    ).get_matches()
}

fn list_dirs(path: &PathBuf, fmt: &DisplayFormat) -> Result<(), io::Error>{
    for entry in std::fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if let Some(meta) = FileMetadata::for_path(&entry.path()) {
                if fmt.should_diplay(&meta) {
                    println!("{}", meta)
                }
            }
        }
    }
    Ok(())
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

pub fn ls_main() -> Result<(), io::Error> {
    let matches = get_arguments();
    let fmt = build_display_fmt(&matches);
    let dirs = matches.values_of("directories");
    print_dirs(dirs, &fmt)
}


#[derive(EnumString)]
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
        color: ColorOption::from_str(&matches.value_of("color").unwrap_or("auto").to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::librb::file::filemeta::tests::setup_test;

    #[test]
    fn test_print_dir() {
        setup_test();
    }
}