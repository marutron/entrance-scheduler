use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "scheme.txt";

    // Открываем файл
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut vectors: Vec<Vec<f64>> = Vec::new();
    let mut current_vector: Vec<f64> = Vec::new();

    // Читаем файл построчно
    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed_line = line.trim();

        // Проверяем, является ли строка процентом
        if trimmed_line.ends_with('%') {
            // Если у нас уже есть текущий вектор, добавляем его в список векторов
            if !current_vector.is_empty() {
                vectors.push(current_vector);
                current_vector = Vec::new();
            }
            // Извлекаем процент и добавляем его в текущий вектор
            if let Ok(percent) = trimmed_line.trim_end_matches('%').parse::<f64>() {
                current_vector.push(percent);
            }
        } else if trimmed_line.starts_with("--->") {
            // Извлекаем числа после стрелочек
            let numbers: Vec<f64> = trimmed_line
                .split(" ---> ")
                .filter_map(|s| s.trim().parse::<f64>().ok())
                .collect();
            // Добавляем числа в текущий вектор
            current_vector.extend(numbers);
        }
    }

    // Добавляем последний вектор, если он не пустой
    if !current_vector.is_empty() {
        vectors.push(current_vector);
    }

    // Печатаем результаты
    for vector in vectors {
        println!("{:?}", vector);
    }
}
