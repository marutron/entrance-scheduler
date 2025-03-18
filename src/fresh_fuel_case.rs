use crate::parser::task::Task;
use crate::tvs::TVS;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct FFC {
    number: u8,
    _1: Rc<RefCell<Option<TVS>>>,
    _2: Rc<RefCell<Option<TVS>>>,
    _3: Rc<RefCell<Option<TVS>>>,
    _4: Rc<RefCell<Option<TVS>>>,
    _5: Rc<RefCell<Option<TVS>>>,
    _6: Rc<RefCell<Option<TVS>>>,
    _7: Rc<RefCell<Option<TVS>>>,
    _8: Rc<RefCell<Option<TVS>>>,
    _9: Rc<RefCell<Option<TVS>>>,
    _10: Rc<RefCell<Option<TVS>>>,
    _11: Rc<RefCell<Option<TVS>>>,
    _12: Rc<RefCell<Option<TVS>>>,
    _13: Rc<RefCell<Option<TVS>>>,
    _14: Rc<RefCell<Option<TVS>>>,
    _15: Rc<RefCell<Option<TVS>>>,
    _16: Rc<RefCell<Option<TVS>>>,
    _17: Rc<RefCell<Option<TVS>>>,
    _18: Rc<RefCell<Option<TVS>>>,
}

impl FFC {
    pub(crate) fn new(number: u8) -> FFC {
        FFC {
            number,
            _1: Rc::new(RefCell::new(None)),
            _2: Rc::new(RefCell::new(None)),
            _3: Rc::new(RefCell::new(None)),
            _4: Rc::new(RefCell::new(None)),
            _5: Rc::new(RefCell::new(None)),
            _6: Rc::new(RefCell::new(None)),
            _7: Rc::new(RefCell::new(None)),
            _8: Rc::new(RefCell::new(None)),
            _9: Rc::new(RefCell::new(None)),
            _10: Rc::new(RefCell::new(None)),
            _11: Rc::new(RefCell::new(None)),
            _12: Rc::new(RefCell::new(None)),
            _13: Rc::new(RefCell::new(None)),
            _14: Rc::new(RefCell::new(None)),
            _15: Rc::new(RefCell::new(None)),
            _16: Rc::new(RefCell::new(None)),
            _17: Rc::new(RefCell::new(None)),
            _18: Rc::new(RefCell::new(None)),
        }
    }

    fn get_16(&mut self) -> Vec<Rc<RefCell<Option<TVS>>>> {
        vec![
            self._1.clone(),
            self._18.clone(),
            self._3.clone(),
            self._16.clone(),
            self._8.clone(),
            self._11.clone(),
            self._2.clone(),
            self._17.clone(),
            self._4.clone(),
            self._15.clone(),
            self._5.clone(),
            self._14.clone(),
            self._6.clone(),
            self._13.clone(),
            self._9.clone(),
            self._10.clone(),
        ]
    }

    fn get_17(&mut self) -> Vec<Rc<RefCell<Option<TVS>>>> {
        vec![
            self._1.clone(),
            self._18.clone(),
            self._3.clone(),
            self._16.clone(),
            self._8.clone(),
            self._11.clone(),
            self._2.clone(),
            self._17.clone(),
            self._7.clone(),
            self._4.clone(),
            self._15.clone(),
            self._5.clone(),
            self._14.clone(),
            self._6.clone(),
            self._13.clone(),
            self._9.clone(),
            self._10.clone(),
        ]
    }
}

impl Display for FFC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ЧСТ № {} (условно) [\n\
                1: {:?},\n \
                2: {:?},\n \
                3: {:?},\n \
                4: {:?},\n \
                5: {:?},\n \
                6: {:?},\n \
                7: {:?},\n \
                8: {:?},\n \
                9: {:?},\n \
                10: {:?},\n \
                11: {:?},\n \
                12: {:?},\n \
                13: {:?},\n \
                14: {:?},\n \
                15: {:?},\n \
                16: {:?},\n \
                17: {:?},\n \
                18: {:?},\n \
            ]",
            self.number,
            self._1.borrow(),
            self._2.borrow(),
            self._3.borrow(),
            self._4.borrow(),
            self._5.borrow(),
            self._6.borrow(),
            self._7.borrow(),
            self._8.borrow(),
            self._9.borrow(),
            self._10.borrow(),
            self._11.borrow(),
            self._12.borrow(),
            self._13.borrow(),
            self._14.borrow(),
            self._15.borrow(),
            self._16.borrow(),
            self._17.borrow(),
            self._18.borrow(),
        )
    }
}

pub fn fill(ffc_number: u8, task: &Task, tvs_pool: &mut Vec<TVS>) -> FFC {
    let mut ffc = FFC::new(ffc_number);
    let mut load_queue = match task.overall_count() {
        16 => ffc.get_16(),
        17 => ffc.get_17(),
        _ => {
            panic!()
        }
    };

    let mut iter = load_queue.into_iter();
    let mut i = 0u8;

    while let Some(cell) = iter.next() {
        let tvs = if i < *task.count_1() {
            get_tvs(task.percent_1().clone(), tvs_pool)
        } else {
            get_tvs(task.percent_2().unwrap(), tvs_pool)
        };

        let mut cell_mut = cell.borrow_mut();
        *cell_mut = Some(tvs);
        i += 1;
    }
    ffc
}

fn get_tvs(percent: f32, tvs_pool: &mut Vec<TVS>) -> TVS {
    for i in 0..tvs_pool.len() {
        if *tvs_pool[i].percent() == percent {
            return tvs_pool.remove(i);
        }
    }
    panic!()
}
