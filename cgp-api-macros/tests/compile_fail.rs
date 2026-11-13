//! Compile-fail coverage for `#[entity_api]` argument and struct validation.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
