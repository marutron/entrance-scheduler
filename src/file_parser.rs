use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Result};

/// Парсит входной файл
///
/// ### Returns
///
/// Hashmap<percent, вектор с номерами позиций]
///
/// ### Panics
///
/// В случае невозможности распарсить строку файла.
/// Выводит в печать её номер и содержание.
pub fn parser(path: &str) -> Result<HashMap<String, Vec<u8>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut position_hash: HashMap<String, Vec<u8>> = HashMap::new();
    let mut vec: Vec<u8> = vec![];
    let mut percent: String = "".to_string();
    let mut i = 0;  // порядковый номер строки

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();
        i += 1;

        if trimmed_line.ends_with('%') {
            if !vec.is_empty() {
                position_hash.insert(percent, vec);
            }
            vec = vec![];
            percent = trimmed_line.trim_end_matches('%').to_string();
        } else if trimmed_line.starts_with("--->") {
            let numbers: Vec<u8> = trimmed_line
                .split("--->")
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .collect();
            vec.push(numbers.into_iter().next().unwrap_or_else(|| {
                panic!("Невозможно распарсить строку № {i} входного файла. Её содержание: [{line}]")
            }))
        }
    }
    if !vec.is_empty() {
        position_hash.insert(percent, vec);
    };
    Ok(position_hash)
}
