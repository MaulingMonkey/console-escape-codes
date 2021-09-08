macro_rules! display {
    ( $($tt:tt)* ) => {
        crate::DisplayFn(move |f| {
            #[allow(unused_imports)] use std::fmt::Write;
            write!(f, $($tt)*)
        })
    };
}
