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

fn get_arguments() -> ArgMatches<'static>  {
    subcommand().get_matches()
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
    use crate::applets::touch::{touch_file, TouchArguments};

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
}