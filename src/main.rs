use std::error::Error;
use crate::file_parser::parser;

mod file_parser;


fn main() -> Result<(), Box<dyn Error>> {
    let path = "scheme.txt";
    let position_hash = parser(path)?;

    for (key, val) in position_hash {
        println!("{key} : {val:?}")
    };
    Ok(())
}
