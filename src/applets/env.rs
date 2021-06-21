use clap::{App, SubCommand, ArgMatches};
use std::io;

pub fn subcommand() -> App<'static, 'static>  {
    SubCommand::with_name("env")
        .about("Print the current environment")
}

fn _env_main(_: Option<&ArgMatches>, writer: &mut impl std::io::Write) -> io::Result<()> {
    for (name, value) in std::env::vars() {
        writeln!(writer, "{name}={value}", name=name, value=value)?;
    }
    Ok(())
}

pub fn env_main(matches: Option<&ArgMatches>) -> Result<(), String>{
    _env_main(matches, &mut std::io::stdout()).or(Err("Failed to run env".to_string()))
}
