#[macro_export]
macro_rules! class_names {
    ($($x:expr_2021),+ $(,)?) => {
        vec![$($x.to_string()),+].join(" ")
    };
}
