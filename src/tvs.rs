use crate::impl_get_field;

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
