use reign_view::parse::*;

mod common;

#[test]
fn test_comment() {
    let element = common::fixture("comment");

    assert_eq!(
        element,
        Element::Comment(Comment {
            content: " This is\n  a comment ".to_string(),
        })
    );
}

#[test]
fn test_attributes() {
    let element = common::fixture("attributes");

    assert_eq!(
        element,
        Element::Tag(Tag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "src".to_string(),
                    value: "example.png".to_string(),
                },
                Attribute {
                    name: "disabled".to_string(),
                    value: "".to_string(),
                },
                Attribute {
                    name: "width".to_string(),
                    value: "200".to_string(),
                },
                Attribute {
                    name: "height".to_string(),
                    value: "100".to_string(),
                }
            ],
            children: vec![],
        })
    );
}

#[test]
fn test_basic() {
    let element = common::fixture("basic");

    assert_eq!(
        element,
        Element::Tag(Tag {
            name: "div".to_string(),
            attrs: vec![],
            children: vec![],
        })
    );
}
