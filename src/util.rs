/// A macro that builds a query tuple by returning the var name and the value.
#[macro_export]
macro_rules! query_tuple {
    ($var: ident) => {
        (stringify!($var), $var)
    };
}
