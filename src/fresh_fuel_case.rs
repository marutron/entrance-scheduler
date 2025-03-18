use crate::parser::task::Task;
use crate::tvs::TVS;
use std::cell::RefCell;
use std::rc::Rc;

pub struct FFC {
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
    pub(crate) fn new() -> FFC {
        FFC {
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

    fn get_16(&mut self) -> [Rc<RefCell<Option<TVS>>>; 16] {
        [
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

    fn get_17(&mut self) -> [Rc<RefCell<Option<TVS>>>; 17] {
        [
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

pub fn fill(task: &Task, tvs_pool: &mut Vec<TVS>) {
    let mut ffc = FFC::new();
    let mut load_queue = ffc.get_16();
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
}

fn get_tvs(percent: f32, tvs_pool: &mut Vec<TVS>) -> TVS {
    for i in 0..tvs_pool.len() {
        if *tvs_pool[i].percent() == percent {
            return tvs_pool.remove(i);
        }
    }
    panic!()
}
