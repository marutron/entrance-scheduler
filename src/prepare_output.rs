use crate::fresh_fuel_case::FFC;
use std::fs::File;
use std::io::Result;
use std::io::Write;

pub fn prepare_output(cases: Vec<FFC>) -> Result<()> {
    let mut output = File::create("output.txt")?;
    for case in cases {
        output.write_all(case.to_string().as_bytes())?;
        output.write_all("\n\n".to_string().as_bytes())?;
    }
    Ok(())
}
