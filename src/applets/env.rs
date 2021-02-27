use clap::{App, Arg, SubCommand, ArgMatches};

pub fn subcommand() -> App<'static, 'static>  {
    SubCommand::with_name("env")
        .about("Print the current environment")
}

fn _env_main(matches: Option<&ArgMatches>, writer: &mut impl std::io::Write) {
    for (name, value) in std::env::vars() {
        writeln!(writer, "{name}={value}", name=name, value=value);
    }
}

pub fn env_main(matches: Option<&ArgMatches>) -> Result<(), String>{
    _env_main(matches, &mut std::io::stdout());
    Ok(())
}
