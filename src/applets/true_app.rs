use clap::{App, Arg, ArgMatches, Values, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("true")
        .about("return success")
}

pub fn true_main(args: Option<&ArgMatches>) -> Result<(), String>{
    Ok(())
}
