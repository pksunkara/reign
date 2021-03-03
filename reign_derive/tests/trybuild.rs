#[rustversion::attr(any(not(stable), before(1.48), since(1.49)), ignore)]
#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
