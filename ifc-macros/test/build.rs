#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("test/variables.rs"); // fake failing to capture warnings
    t.pass("test/format_lowvars.rs");
    t.compile_fail("test/format_highvars.rs");
    t.pass("test/define_high_as_low.rs");
    t.compile_fail("test/define_low_as_high.rs");
    t.pass("test/assign_untyped_to_untyped.rs");
    t.pass("test/assign_low_to_untyped.rs");
    t.compile_fail("test/assign_high_to_untyped.rs");
    t.pass("test/assign_untyped_to_low.rs");
    t.pass("test/assign_low_to_low.rs");
    t.compile_fail("test/assign_high_to_low.rs");
    t.pass("test/assign_untyped_to_high.rs");
    t.pass("test/assign_low_to_high.rs");
    t.pass("test/assign_high_to_high.rs");
    t.pass("test/pass_low_to_fn.rs");
    t.pass("test/pass_high_unsafe_to_fn.rs");
    t.compile_fail("test/pass_high_to_fn.rs");
    t.compile_fail("test/pass_high_mut_to_fn.rs");
    t.compile_fail("test/pass_high_ref_to_fn.rs");
    t.compile_fail("test/unused_attribute.rs");
}
