/// A macro that builds a query tuple by taking the var name and the value.
#[macro_export]
macro_rules! query_tuple {
    ($var: ident) => {
        (stringify!($var), $var)
    }
}