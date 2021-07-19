macro_rules! extract {
    ($expression:expr, $pattern:pat => $getter:expr) => {
        extract!($expression, $pattern => $getter, "failed to extract")
    };
    ($expression:expr, $pattern:pat => $getter:expr, $err:literal) => {
        match $expression {
            $pattern => $getter,
            _ => unreachable!($err),
        }
    };
}
