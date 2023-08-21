macro_rules! assert_pretty_syn_snapshot {
    ($ty:ty => $code:literal) => {
        let parsed = &syn::parse_str::<$ty>($code).unwrap();
        let format = format!(
            "pretty:\n{}\n\ncolor:\n{}\n",
            dbg_pls::pretty(&parsed),
            dbg_pls::color(&parsed)
        );
        insta::assert_snapshot!(insta::_macro_support::AutoName, &format, $code);
    };
}

#[test]
fn expr_array() {
    assert_pretty_syn_snapshot!(syn::Expr => r#"
            [
                "Hello, World! I am a long string",
                420,
                "Wait, you can't mix and match types in arrays, is this python?",
                69,
                "Nice."
            ]
        "#);
}

#[test]
fn qself() {
    assert_pretty_syn_snapshot!(syn::Expr => "<Foo as Bar::<<Baz as Qux>::Bang>>::Boo");
}

#[test]
fn item_macro() {
    assert_pretty_syn_snapshot!(syn::Item => r#"
            macro_rules! assert_pretty_syn_snapshot {
                ($ty:ty => $code:literal) => {
                    insta::assert_snapshot!(
                        insta::_macro_support::AutoName,
                        &crate::pretty(&syn::parse_str::<$ty>($code).unwrap()).to_string(),
                        $code
                    );
                };
            }
        "#);
}
