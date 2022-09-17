use std::str::FromStr;

#[test]
fn tests() {
    let ts = proc_macro2::TokenStream::from_str("{1, right:{2, left: 3}}").unwrap();
    dbg!(ts);
    let t = trybuild::TestCases::new();
    t.pass("tests/tree.rs");
    //t.pass("tests/02-parse-body.rs");
    //t.compile_fail("tests/03-expand-four-errors.rs");
    //t.pass("tests/04-paste-ident.rs");
    //t.pass("tests/05-repeat-section.rs");
    //t.pass("tests/06-init-array.rs");
    //t.pass("tests/07-inclusive-range.rs");
    //t.compile_fail("tests/08-ident-span.rs");
    //t.pass("tests/09-interaction-with-macrorules.rs");
}
