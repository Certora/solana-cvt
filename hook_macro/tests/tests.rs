#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/expand/test_hook_error.rs");
}