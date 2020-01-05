mod common;

#[test]
fn test_comment() {
    common::parse_pass("comment");
}

#[test]
fn test_attributes() {
    common::parse_pass("attributes");
}

#[test]
fn test_dynamic_attribute() {
    common::parse_pass("dynamic_attribute");
}

#[test]
fn test_basic() {
    common::parse_pass("basic");
}

#[test]
fn test_attribute_good() {
    common::parse_pass("attribute_good");
}

#[test]
fn test_multi_roots() {
    common::parse_fail("multi_roots");
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

// TODO: More tests (this,slots,component,expr)
#[test]
fn test_interpolation_bad() {
    // common::parse_fail("interpolation_bad");
}
