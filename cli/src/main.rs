pub mod input_user;

use ansi_term::Colour;
use crate::input_user::new_input;
use circom::circom_compiler;

fn main() {
    let result = start();
    if result.is_err() {
        eprintln!("{}", Colour::Red.paint("previous errors were found"));
        std::process::exit(1);
    } else {
        println!("{}", Colour::Green.paint("Everything went okay, circom safe"));
        //std::process::exit(0);
    }
}
fn start() -> Result<(), ()> {
    let user_input = new_input()?;
    circom_compiler(user_input)
}
