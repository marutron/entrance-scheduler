use crate::impl_get_field;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TVS {
    number: String,
    container: String,
    percent: f32,
}

impl TVS {
    pub fn new(number: String, container: String, percent: f32) -> Self {
        TVS {
            number,
            container,
            percent,
        }
    }

    impl_get_field![
        number -> String,
        container -> String,
        percent -> f32
    ];
}

impl Display for TVS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ТВС [number: {}, container: {}]",
            self.number, self.container,
        )
    }
}
