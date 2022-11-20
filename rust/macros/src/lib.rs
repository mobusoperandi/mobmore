#[macro_export]
macro_rules! hashmap {
    (,) => { ::std::compile_error!() };

    ($($key:literal => $value:expr),* $(,)?) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
    }};
}
