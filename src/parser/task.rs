use crate::impl_get_field;

#[derive(Debug)]
pub struct Task {
    percent_1: f32,
    count_1: u8,
    percent_2: Option<f32>,
    count_2: u8,
}

impl Task {
    pub fn new(task_str: &str) -> Self {
        let pair_vec: Vec<String> = task_str
            .split_whitespace()
            .filter_map(|s| Option::from(s.trim().to_string()))
            .collect();
        let sort_1: Vec<String> = pair_vec[0]
            .split("-")
            .filter_map(|s| Option::from(s.trim().to_string()))
            .collect();
        let sort_2 = if let Some(pair) = pair_vec.get(1) {
            let sort_2: Vec<String> = pair
                .split("-")
                .filter_map(|s| Option::from(s.trim().to_string()))
                .collect();
            (
                Some(sort_2[0].parse::<f32>().unwrap()),
                sort_2[1].parse::<u8>().unwrap(),
            )
        } else {
            (None, 0)
        };

        Task {
            percent_1: sort_1[0].parse::<f32>().unwrap(),
            count_1: sort_1[1]
                .parse::<u8>()
                .expect("Невозможно распарсить количество ТВС в задании"),
            percent_2: sort_2.0,
            count_2: sort_2.1,
        }
    }

    pub fn overall_count(&self) -> u8 {
        self.count_1 + self.count_2
    }

    impl_get_field![
        percent_1-> f32,
        count_1-> u8,
        percent_2-> Option<f32>,
        count_2-> u8
    ];
}
