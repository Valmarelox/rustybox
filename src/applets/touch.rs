use clap::{App, Arg, ArgMatches, Values};
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

fn get_arguments() -> ArgMatches<'static>  {
    App::new("touch")
        .version("0.0.1")
        .author("Efi Weiss <valmarelox@gmail.com>")
        .about("A not that busy (yet!) and still a bit rusty box")
        .arg(
            Arg::with_name("create").short("-c").long("--no-create").takes_value(false).help("don't create file")
        ).arg(
            Arg::with_name("files").multiple(true)
    ).get_matches()
}

pub fn touch_main() -> Result<(), io::Error>{
    let args = get_arguments();
    let ta = TouchArguments { create_file: !args.is_present("create")};
    for f in args.values_of("files").ok_or(io::Error::from_raw_os_error(133))? {
        touch_file(f.to_string(), &ta)?;
    }
    Ok(())
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