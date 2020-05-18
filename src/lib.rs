macro_rules! path_of {
    ($p: path) => {{
        let _ = || {
            use $p;
        };

        stringify!($p)
    }};
}
