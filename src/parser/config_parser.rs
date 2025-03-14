use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Парсит конфиг программы, полученный из файла config.txt
pub fn parse_config() -> Result<String> {
    let file = File::open("config.txt")?;
    let reader = BufReader::new(file);

    let mut line_iter = reader.lines();

    // Получаем строки с информацией
    let path_line = line_iter
        .next()
        .unwrap_or_else(|| panic!("Невозможно распарсить файл config.txt, он пуст!"))?;

    let vec: Vec<String> = path_line
        .split("=")
        .filter_map(|s| Option::from(s.trim().to_string()))
        .collect();
    let path = vec[vec.len() - 1].clone();

    Ok(path)
}
