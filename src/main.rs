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
    // вызывается в случае отсутствия готовых tasks
    // let position_hash = parse_file(path)?;
    let mut tvs_pool = parse_tvs(&tvs_path)?;
    let mut cases = vec![];

    if let Some(tasks) = tasks {
        let mut ffc_num = 1;
        for task in tasks {
            let ffc = fill(ffc_num.clone(), &task, &mut tvs_pool);
            println!("{ffc}");
            cases.push(ffc);
            ffc_num += 1
        }
    }
    Ok(())
}
