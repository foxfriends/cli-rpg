#[macro_export]
macro_rules! __downcast {
    ($any:ident, else => $end:expr $(,)?) => {
        break $end
    };
    ($any:ident, $name:ident : $t:ty => $eval:expr, $($rest:tt)+) => {{
        if let Some($name) = $any.downcast_ref::<$t>() {
            break $eval;
        }
        __downcast!($iany, $($rest)+)
    }};
    ($any:ident, $pattern:pat => $eval:expr, $($rest:tt)+) => {{
        if let Some($pattern) = $any.downcast_ref() {
            break $eval;
        }
        __downcast!($any, $($rest)+)
    }};
}

#[macro_export]
macro_rules! downcast {
    ($anyval:expr, { $($rest:tt)+ }) => {{
        let any = $anyval;
        loop {
            __downcast!(any, $($rest)+)
        }
    }};
}
