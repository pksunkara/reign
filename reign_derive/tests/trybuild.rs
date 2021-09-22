#[rustversion::attr(any(not(stable), before(1.54), since(1.55)), ignore)]
#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
