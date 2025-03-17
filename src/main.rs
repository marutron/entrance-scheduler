use crate::mapper::orbit_mapper::convert_to_queue;
use crate::parser::config_parser::parse_config;
use crate::parser::file_parser::parse_file;
use crate::parser::tvs::parse_tvs;
use std::error::Error;

mod fresh_fuel_case;
pub mod macros;
mod mapper;
mod parser;
pub mod tvs;

fn main() -> Result<(), Box<dyn Error>> {
    let (path, tvs_path, tasks) = parse_config()?;
    let position_hash = parse_file(path)?;
    let tvs_pool = parse_tvs(&tvs_path);

    for (key, val) in &position_hash {
        println!("{key} : {val:?}")
    }

    if let None = tasks {
        let queue = convert_to_queue(&position_hash);
    }

    if let Some(tasks) = tasks {
        println!("{tasks:?}")
    }

    Ok(())
}
