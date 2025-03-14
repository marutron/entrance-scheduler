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
