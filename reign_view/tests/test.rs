use reign_view::parse::*;

mod common;

#[test]
fn test_comment() {
    let element = common::fixture("comment");

    assert_eq!(
        element,
        Node::Comment(Comment {
            content: " This is\n  a comment ".to_string(),
        })
    );
}

#[test]
fn test_attributes() {
    let element = common::fixture("attributes");

    assert_eq!(
        element,
        Node::Element(Element {
            name: "div".to_string(),
            attrs: vec![
                Attribute::Normal(NormalAttribute {
                    name: "src".to_string(),
                    value: "example.png".to_string(),
                }),
                Attribute::Normal(NormalAttribute {
                    name: "disabled".to_string(),
                    value: "".to_string(),
                }),
                Attribute::Normal(NormalAttribute {
                    name: "width".to_string(),
                    value: "200".to_string(),
                }),
                Attribute::Normal(NormalAttribute {
                    name: "height".to_string(),
                    value: "100".to_string(),
                }),
            ],
            children: vec![],
        })
    );
}

#[test]
fn test_dynamic_attribute() {
    let element = common::fixture("dynamic_attribute");

    assert_eq!(
        element,
        Node::Element(Element {
            name: "div".to_string(),
            attrs: vec![Attribute::Dynamic(DynamicAttribute {
                symbol: ":".to_string(),
                prefix: "dy".to_string(),
                expr: "var[\"key\"]".to_string(),
                suffix: "ic".to_string(),
                value: "format!(\"{}_concat\", ident)".to_string(),
            })],
            children: vec![],
        })
    );
}

#[test]
fn test_basic() {
    let element = common::fixture("basic");

    assert_eq!(
        element,
        Node::Element(Element {
            name: "div".to_string(),
            attrs: vec![],
            children: vec![
                Node::Text(Text {
                    content: "\n  ".to_string()
                }),
                Node::Element(Element {
                    name: "h1".to_string(),
                    attrs: vec![],
                    children: vec![Node::Text(Text {
                        content: "Hello".to_string()
                    })]
                }),
                Node::Text(Text {
                    content: "\n  ".to_string()
                }),
                Node::Element(Element {
                    name: "br".to_string(),
                    attrs: vec![],
                    children: vec![]
                }),
                Node::Text(Text {
                    content: "\n  ".to_string()
                }),
                Node::Element(Element {
                    name: "p".to_string(),
                    attrs: vec![],
                    children: vec![Node::Text(Text {
                        content: "Lorem Ipsum".to_string()
                    })]
                }),
                Node::Text(Text {
                    content: "\n".to_string()
                })
            ],
        })
    );
}
