use crate::impl_get_field;
use crate::parser::task::Task;
use crate::tvs::TVS;
use std::collections::HashSet;

struct FFC {
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
    pub fn parse_from_task(task: Task, tvs_pool: HashSet<TVS>) {}

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
}
