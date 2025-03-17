use crate::tvs::TVS;

#[macro_export]
macro_rules! impl_get_field {
    ($($field: ident -> $r_type: ty), *) => {
            $(
                pub fn $field (&self) -> &$r_type {
                    &self.$field
                }
            )*
    };
}

#[macro_export]
macro_rules! impl_set_tvs_field {
    ($($method_name: ident, $field: ident), *) => {
        $(
            fn $method_name(&mut self, tvs: TVS) {
                self.$field = Some(tvs)
            }
        )*
    };
}
