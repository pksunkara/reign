mod common;

#[test]
fn test_comment() {
    common::parse_pass("comment");
}

#[test]
fn test_basic() {
    common::parse_pass("basic");
}

#[test]
fn test_multi_roots() {
    common::parse_fail("multi_roots");
}

#[test]
fn test_attribute_good() {
    common::parse_pass("attribute_good");
}

#[test]
fn test_attribute_bad() {
    common::parse_fail("attribute_bad");
}

#[test]
fn test_doctype() {
    common::parse_pass("doctype");
}

#[test]
fn test_interpolation_good() {
    common::parse_pass("interpolation_good");
}

#[test]
fn test_interpolation_bad() {
    common::parse_fail("interpolation_bad");
}

#[test]
fn test_for() {
    common::parse_pass("for");
}

#[test]
fn test_if() {
    common::parse_pass("if");
}

#[test]
fn test_normal_attribute() {
    common::parse_pass("normal_attribute");
}

#[test]
fn test_variable_attribute() {
    common::parse_pass("variable_attribute");
}

#[test]
fn test_dynamic_attribute() {
    common::parse_pass("dynamic_attribute");
}

#[test]
fn test_component() {
    common::parse_pass("component");
}

#[test]
fn test_component_if() {
    common::parse_pass("component_if");
}

#[test]
fn test_slot() {
    common::parse_pass("slot");
}

// #[test]
// fn test_else_without_if() {
//     common::parse_fail("else_without_if");
// }
