use crate::{impl_get_field, impl_set_tvs_field};
use crate::parser::task::Task;
use crate::tvs::TVS;
use std::collections::HashSet;

pub struct FFC {
    _1: Option<TVS>,
    _2: Option<TVS>,
    _3: Option<TVS>,
    _4: Option<TVS>,
    _5: Option<TVS>,
    _6: Option<TVS>,
    _7: Option<TVS>,
    _8: Option<TVS>,
    _9: Option<TVS>,
    _10: Option<TVS>,
    _11: Option<TVS>,
    _12: Option<TVS>,
    _13: Option<TVS>,
    _14: Option<TVS>,
    _15: Option<TVS>,
    _16: Option<TVS>,
    _17: Option<TVS>,
    _18: Option<TVS>,
}

impl FFC {
    fn set_16_iter(&mut self) -> Vec<Box<dyn FnMut(TVS)>> {
        vec![
            Box::new(move | tvs | self.set_1(tvs)),
            Box::new(move | tvs | self.set_18(tvs)),
            Box::new(move | tvs | self.set_3(tvs)),
            Box::new(move | tvs | self.set_16(tvs)),
            Box::new(move | tvs | self.set_8(tvs)),
            Box::new(move | tvs | self.set_11(tvs)),
            Box::new(move | tvs | self.set_2(tvs)),
            Box::new(move | tvs | self.set_17(tvs)),
            Box::new(move | tvs | self.set_4(tvs)),
            Box::new(move | tvs | self.set_15(tvs)),
            Box::new(move | tvs | self.set_5(tvs)),
            Box::new(move | tvs | self.set_14(tvs)),
            Box::new(move | tvs | self.set_6(tvs)),
            Box::new(move | tvs | self.set_13(tvs)),
            Box::new(move | tvs | self.set_9(tvs)),
            Box::new(move | tvs | self.set_10(tvs)),
        ]
    }

    fn set_17_iter(&mut self) -> Vec<Box<dyn FnMut(TVS)>> {
        vec![
            Box::new(move | tvs | self.set_1(tvs)),
            Box::new(move | tvs | self.set_18(tvs)),
            Box::new(move | tvs | self.set_3(tvs)),
            Box::new(move | tvs | self.set_16(tvs)),
            Box::new(move | tvs | self.set_8(tvs)),
            Box::new(move | tvs | self.set_11(tvs)),
            Box::new(move | tvs | self.set_2(tvs)),
            Box::new(move | tvs | self.set_17(tvs)),
            Box::new(move | tvs | self.set_7(tvs)),
            Box::new(move | tvs | self.set_4(tvs)),
            Box::new(move | tvs | self.set_15(tvs)),
            Box::new(move | tvs | self.set_5(tvs)),
            Box::new(move | tvs | self.set_14(tvs)),
            Box::new(move | tvs | self.set_6(tvs)),
            Box::new(move | tvs | self.set_13(tvs)),
            Box::new(move | tvs | self.set_9(tvs)),
            Box::new(move | tvs | self.set_10(tvs)),
        ]
    }

    fn new () -> FFC {
        FFC {
            _1: None,
            _2: None,
            _3: None,
            _4: None,
            _5: None,
            _6: None,
            _7: None,
            _8: None,
            _9: None,
            _10: None,
            _11: None,
            _12: None,
            _13: None,
            _14: None,
            _15: None,
            _16: None,
            _17: None,
            _18: None,
        }
    }

    pub fn fill(task: Task, mut tvs_pool: HashSet<TVS>) {
        let mut ffc = FFC::new();
        let overall_count = task.overall_count();
        let mut i = 0;
        let mut iter = match overall_count {
            16 => {ffc.set_17_iter().iter()}
            17 => {ffc.set_17_iter().iter()}
            _ => {panic!("Инициализирован Task на ЧСТ с количеством операций: {overall_count}")}
        };
        while i < overall_count {
            let tvs: TVS = {
                let mut tvs = TVS::new("".to_string(), "".to_string(), 0.0);
                for item in &tvs_pool {
                    if item.percent() == task.percent_1() {
                        tvs = tvs_pool.take(&item).unwrap();
                    }
                };
                assert_ne!(*tvs.percent(), 0.0);
                tvs
            };
            iter.next().unwrap()(tvs);
            i += 1;
        }
    }

    impl_get_field![
    _1 -> Option<TVS>,
    _2 -> Option<TVS>,
    _3 -> Option<TVS>,
    _4 -> Option<TVS>,
    _5 -> Option<TVS>,
    _6 -> Option<TVS>,
    _7 -> Option<TVS>,
    _8 -> Option<TVS>,
    _9 -> Option<TVS>,
    _10 -> Option<TVS>,
    _11 -> Option<TVS>,
    _12 -> Option<TVS>,
    _13 -> Option<TVS>,
    _14 -> Option<TVS>,
    _15 -> Option<TVS>,
    _16 -> Option<TVS>,
    _17 -> Option<TVS>,
    _18 -> Option<TVS>
    ];

    impl_set_tvs_field![
        set_1, _1,
        set_2, _2,
        set_3, _3,
        set_4, _4,
        set_5, _5,
        set_6, _6,
        set_7, _7,
        set_8, _8,
        set_9, _9,
        set_10, _10,
        set_11, _11,
        set_12, _12,
        set_13, _13,
        set_14, _14,
        set_15, _15,
        set_16, _16,
        set_17, _17,
        set_18, _18
    ];
}
