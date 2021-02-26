use clap::{App, Arg, ArgMatches, Values, SubCommand};
use std::fs::File;
use std::io;

struct TouchArguments {
    create_file: bool,
}

fn touch_file(name: String, args: &TouchArguments) -> Result<(), io::Error> {
    let f = match args.create_file {
        true => File::create(name)?,
        false => File::open(name)?,
    };
    Ok(())
}

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("touch")
        .about("Touch a file")
        .arg(
            Arg::with_name("create").short("-c").long("--no-create").takes_value(false).help("don't create file")
        ).arg(
        Arg::with_name("files").multiple(true).index(1).required(true)
    )
}

fn touch_files(files: Values, args: &TouchArguments) -> Result<(), String>{
    for f in  files {
        touch_file(f.to_string(), args).or(Err(format!("Failed to touch {}", f).to_string()))?;
    }
    Ok(())
}

pub fn touch_main(args: Option<&ArgMatches>) -> Result<(), String>{
    // OK because argument is required
    let args = args.unwrap();
    let ta = TouchArguments { create_file: !args.is_present("create")};
    let files = args.values_of("files").unwrap();
    touch_files(files, &ta)
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::{touch_main, touch_file, TouchArguments, subcommand};
    use std::ffi::{OsString, OsStr};
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn test_create_touch() {
        let res = touch_file("/tmp/should_create".to_string(), &TouchArguments { create_file: true }).err();
        assert!(res.is_none());
        let res = touch_file("/tmp/should_fail".to_string(), &TouchArguments { create_file: false }).err().unwrap();
        // Should ENOENT (2)
        assert_eq!(res.raw_os_error().unwrap(), 2);
        let res = touch_file("/tmp/should_create".to_string(), &TouchArguments { create_file: false }).err();
        assert!(res.is_none());
    }

    #[test]
    fn test_subcommand_fail_to_create() {
        let name = "hello";
        let args: [&OsStr; 3] = [OsStr::new("touch"), OsStr::new("-c"), OsStr::new(name)];
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        assert!(matches.is_present("create"));
        assert_eq!(touch_main(Some(&matches)).unwrap_err(), format!("Failed to touch {}", name));
    }

    #[test]
    fn test_subcommand_create() {
        let name = "/tmp/chello";
        let args: [&OsStr; 2] = [OsStr::new("touch"), OsStr::new(name)];
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        Command::new("sh")
            .arg("-c")
            .arg(format!("rm -f {name}", name=name))
            .output()
            .expect("failed to execute process");
        assert!(!Path::new(name).exists());
        assert!(!matches.is_present("create"));
        assert!(touch_main(Some(&matches)).is_ok());
        assert!(Path::new(name).exists());
    }
}