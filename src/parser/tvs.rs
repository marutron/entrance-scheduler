use crate::tvs::TVS;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_tvs(path: &str) -> std::io::Result<Vec<TVS>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut i = 0; // порядковый номер строки
    let mut pool = Vec::with_capacity(67);

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();
        i += 1;

        let vec: Vec<String> = trimmed_line
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();

        let percent = &vec[0][1..4].parse::<f32>().unwrap() / 100.0;
        let tvs = TVS::new(vec[0].clone(), vec[1].clone(), percent);
        pool.push(tvs);
    }
    Ok(pool)
}
