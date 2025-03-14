use crate::impl_get_field;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Index};

#[derive(Debug)]
pub struct Idx {
    percent: String,
    cell_num: u8, // номер ячейки
    index: usize, // найденный индекс
}

impl Idx {
    impl_get_field![
        percent -> String,
        cell_num -> u8,
        index -> usize
    ];
}

impl Display for Idx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Idx [percent: {}, cell_num: {}, index: {}]",
            self.percent, self.cell_num, self.index
        )
    }
}

pub fn convert_to_queue(position_hash: &HashMap<String, Vec<u8>>) -> Vec<Idx> {
    let mut queue: Vec<Idx> = vec![];
    for (percent, vec) in position_hash {
        for cell_num in vec {
            let idx = map_orbit(percent.clone(), *cell_num);
            queue.push(idx)
        }
    }
    queue.sort_by(|a, b| a.index().cmp(&b.index()));
    queue
}

#[inline]
fn map_orbit(percent: String, cell_num: u8) -> Idx {
    let mapper = orbit_mapper();
    let mut index = 0usize;
    for (orbit_num, vec) in mapper {
        for i in 0..vec.len() {
            if cell_num == vec[i] {
                index = orbit_num as usize * i;
                return Idx {
                    percent,
                    cell_num: cell_num.clone(),
                    index,
                };
            }
        }
    }
    panic!("Невозможно найти ячейку с номером {cell_num} на картограмме. Проверьте входной файл");
}

#[inline]
fn orbit_mapper() -> HashMap<u8, Vec<u8>> {
    let mut hash = HashMap::new();
    hash.insert(1, vec![82]);
    hash.insert(2, vec![83, 96, 95, 81, 68, 69]);
    hash.insert(3, vec![84, 97, 110, 109, 108, 94, 80, 67, 54, 55, 56, 70]);
    hash.insert(
        4,
        vec![
            85, 98, 111, 123, 122, 121, 120, 107, 93, 79, 66, 53, 41, 42, 43, 44, 57, 71,
        ],
    );
    hash.insert(
        5,
        vec![
            86, 99, 112, 124, 135, 134, 133, 132, 131, 119, 106, 92, 78, 65, 52, 40, 29, 30, 31,
            32, 33, 45, 58, 72,
        ],
    );
    hash.insert(
        6,
        vec![
            87, 100, 113, 125, 136, 146, 145, 144, 143, 142, 141, 130, 118, 105, 91, 77, 64, 51,
            39, 28, 18, 19, 20, 21, 22, 23, 34, 46, 59, 73,
        ],
    );
    hash.insert(
        7,
        vec![
            88, 101, 114, 126, 137, 147, 156, 155, 154, 153, 152, 151, 150, 140, 129, 117, 104, 90,
            76, 63, 50, 38, 27, 17, 8, 9, 10, 11, 12, 13, 14, 24, 35, 47, 60, 74,
        ],
    );
    hash.insert(
        8,
        vec![
            102, 115, 127, 138, 148, 157, 163, 162, 161, 160, 159, 158, 149, 139, 128, 116, 103,
            89, 62, 49, 37, 26, 16, 7, 1, 2, 3, 4, 5, 6, 15, 25, 36, 48, 61, 75,
        ],
    );
    hash
}
