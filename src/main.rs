use crate::parser::config_parser::parse_config;
use crate::parser::file_parser::parse_file;
use std::error::Error;

mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let path = parse_config()?;
    let position_hash = parse_file(path)?;

    for (key, val) in position_hash {
        println!("{key} : {val:?}")
    }
    Ok(())
}
