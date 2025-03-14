#[derive(Debug)]
pub struct Task {
    percent_1: String,
    count_1: u8,
    percent_2: Option<String>,
    count_2: u8,
}

impl Task {
    pub fn new(task_str: &str) -> Self {
        let sort_vec: Vec<String> = task_str
            .split(" ")
            .filter_map(|s| Option::from(s.trim().to_string()))
            .collect();
        let sort_1: Vec<String> = sort_vec[0]
            .split("-")
            .filter_map(|s| Option::from(s.trim().to_string()))
            .collect();
        let sort_2: Vec<String> = sort_vec[0]
            .split("-")
            .filter_map(|s| Option::from(s.trim().to_string()))
            .collect();
        // todo sort_2 - Option

        Task {
            percent_1: sort_1[0].clone(),
            count_1: sort_1[1]
                .parse::<u8>()
                .expect("Невозможно распарсить количество ТВС в задании"),
            percent_2: Some(sort_2[0].clone()),
            count_2: sort_2[1].parse::<u8>().unwrap(),
        }
    }
}
