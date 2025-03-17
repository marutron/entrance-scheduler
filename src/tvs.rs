use crate::impl_get_field;
use std::hash::{Hash, Hasher};

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

impl PartialEq<Self> for TVS {
    fn eq(&self, other: &Self) -> bool {
        if self.number == other.number {
            true
        } else {
            false
        }
    }
}

impl Eq for TVS {}

impl Hash for TVS {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}
