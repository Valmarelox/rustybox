mod librb;
mod applets;
mod core;

use crate::applets::ls::ls_main;
use crate::applets::touch::{touch_main};
use clap::App;
use crate::core::args::add_generic_info;
use crate::applets::env::env_main;
use crate::applets::cat::cat_main;
use crate::applets::sleep::sleep_main;


extern crate chrono;
extern crate strum;
#[macro_use] extern crate strum_macros;
#[macro_use]
extern crate bitflags;

extern crate num_enum;

fn get_app() -> App<'static, 'static> {
    add_generic_info(App::new("rustybox"))
        .subcommand(applets::ls::subcommand())
        .subcommand(applets::touch::subcommand())
        .subcommand(applets::env::subcommand())
        .subcommand(applets::cat::subcommand())
        .subcommand(applets::sleep::subcommand())

}

fn main() -> Result<(), String> {
    let mut app = get_app();
    let args = app.get_matches_from_safe_borrow(std::env::args());
    if let Ok(args) =  args {
        let (cmd, args) = args.subcommand();
        match cmd {
            "touch" => touch_main(args),
            "ls" => ls_main(args),
            "env" => env_main(args),
            "cat" => cat_main(args),
            "sleep" => sleep_main(args),
            cmd => {
                app.print_long_help().or(Err("Failed to print help"))?;
                Err(format!("Invalid Command {}", cmd).to_string())
            },
        }
    } else {
        println!("{}", args.unwrap_err().message);
        Ok(())
    }
}
