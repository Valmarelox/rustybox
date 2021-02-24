mod librb;
mod applets;
mod core;

use crate::applets::ls::ls_main;
use crate::applets::touch::{touch_main};
use clap::{App, ArgSettings, AppSettings};
use crate::core::args::add_generic_info;


extern crate chrono;
extern crate strum;
#[macro_use] extern crate strum_macros;
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_enum;

fn main() -> Result<(), String> {
    let mut app = add_generic_info(App::new("rustybox"))
        .subcommand(applets::ls::subcommand())
        .subcommand(applets::touch::subcommand());
    let args = app.get_matches_from_safe_borrow(std::env::args());
    if let Ok(args) =  args {
        if let (cmd, args) = args.subcommand() {
            match cmd {
                "touch" => touch_main(args),
                "ls" => ls_main(args),
                x => {
                    app.print_long_help();
                    Err("Invalid Command".to_string())
                },
            }
        } else {
            Err("How did we get here".to_string())
        }
    } else {
        println!("{}", args.unwrap_err().message);
        Ok(())
    }
}
