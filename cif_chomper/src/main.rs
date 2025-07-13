use log::LevelFilter;
use std::{error::Error as StdError, fs};
mod logging;
mod model;
mod parser;

fn main() -> Result<(), Box<dyn StdError>> {
    log::set_logger(&logging::LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Failed to set up logger!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() < 2 {
        println!("Supply an arg!");
        return Ok(());
    }
    let text = fs::read_to_string(&args[1])?;
    let orig_length = text.len();
    let parse = parser::cif2_file(text.as_str()).unwrap();
    dbg!(parse);
    Ok(())
}
