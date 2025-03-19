use crate::parser::task::Task;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Парсит конфиг программы, полученный из файла config.txt
pub fn parse_config() -> Result<(String, String, Option<[Task; 4]>)> {
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

    let tvs_line = line_iter.next().unwrap()?;
    let vec: Vec<String> = tvs_line
        .split("=")
        .filter_map(|s| Option::from(s.trim().to_string()))
        .collect();
    let tvs_path = vec[vec.len() - 1].clone();

    line_iter.next(); // blank line

    let mut tasks = None;
    if let Some(queue_line) = line_iter.next() {
        if let Ok(queue_line) = queue_line {
            if queue_line == "queue:".to_string() {
                let first = line_iter
                    .next()
                    .unwrap_or_else(|| panic!("Отсутствует первая строка после queue"))?;
                let second = line_iter
                    .next()
                    .unwrap_or_else(|| panic!("Отсутствует вторая строка после queue"))?;
                let third = line_iter
                    .next()
                    .unwrap_or_else(|| panic!("Отсутствует третья строка после queue"))?;
                let fourth = line_iter
                    .next()
                    .unwrap_or_else(|| panic!("Отсутствует четвертая строка после queue"))?;

                tasks = Some([
                    Task::new(first.trim()),
                    Task::new(second.trim()),
                    Task::new(third.trim()),
                    Task::new(fourth.trim()),
                ])
            }
        }
    }
    Ok((path, tvs_path, tasks))
}
