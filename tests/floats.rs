macro_rules! float_tests {
    ($ty:ident) => {
        mod $ty {
            #[test]
            fn non_finite() {
                for (value, expected) in [
                    ($ty::INFINITY, concat!(stringify!($ty), "::INFINITY")),
                    (
                        $ty::NEG_INFINITY,
                        concat!(stringify!($ty), "::NEG_INFINITY"),
                    ),
                    ($ty::NAN, concat!(stringify!($ty), "::NAN")),
                    (-$ty::NAN, concat!(stringify!($ty), "::NAN")),
                ] {
                    let output = dbg_pls::pretty(&value).to_string();
                    assert_eq!(output, expected);
                    syn::parse_str::<syn::Expr>(&output).unwrap();
                    // Highlighting uses the same expression, and must not panic either.
                    assert!(!dbg_pls::color(&value).to_string().is_empty());
                }
            }

            #[test]
            fn finite() {
                for (value, expected) in [
                    (0.0 as $ty, "0.0"),
                    (-0.0 as $ty, "-0.0"),
                    (1.5 as $ty, "1.5"),
                    (-1.5 as $ty, "-1.5"),
                ] {
                    assert_eq!(dbg_pls::pretty(&value).to_string(), expected);
                }
            }
        }
    };
}

float_tests!(f32);
float_tests!(f64);
