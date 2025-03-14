use crate::mapper::orbit_mapper::convert_to_queue;
use crate::parser::config_parser::parse_config;
use crate::parser::file_parser::parse_file;
use std::error::Error;

pub mod macros;
mod mapper;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let path = parse_config()?;
    let position_hash = parse_file(path)?;

    for (key, val) in &position_hash {
        println!("{key} : {val:?}")
    }

    let queue = convert_to_queue(&position_hash);

    for itm in &queue {
        println!("{itm}");
    }

    println!("{}, {}", position_hash.values().len(), queue.len());
    Ok(())
}
