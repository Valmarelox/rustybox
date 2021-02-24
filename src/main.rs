mod librb;
mod applets;

use crate::applets::ls::ls_main;
use crate::applets::touch::touch_main;


extern crate chrono;
extern crate strum;
#[macro_use] extern crate strum_macros;
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_enum;

fn main() -> Result<(), std::io::Error> {
    //ls_main()
    touch_main()
}
