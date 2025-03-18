use crate::fresh_fuel_case::fill;
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
    let mut tvs_pool = parse_tvs(&tvs_path)?;

    if let Some(tasks) = tasks {
        for task in tasks {
            fill(&task, &mut tvs_pool);
        }
    }

    for (key, val) in &position_hash {
        println!("{key} : {val:?}")
    }

    Ok(())
}
