extern crate native_ui;

use native_ui::style::{
    Attribute, AttributeCompare, Element, Property, PropertyValue, Pseudo, Select,
    Style, Stylesheet,
};

fn main() {
    let stylesheet = Stylesheet {
        styles: vec![Style {
            selector: Select::Child(
                Box::new(Select::Element(
                    Element {
                        tag: Some("div".to_owned()),
                        classes: vec!["bold_underline".to_owned()],
                        id: Some("sample".to_owned()),
                        attributes: vec![
                            Attribute {
                                name: "active".to_owned(),
                                compare: AttributeCompare::Exists,
                                value: None,
                                case_sensitive: true,
                            },
                            Attribute {
                                name: "id".to_owned(),
                                compare: AttributeCompare::StartsWith,
                                value: Some("sample".to_owned()),
                                case_sensitive: true,
                            },
                        ],
                    },
                    Pseudo::Class("hover".to_owned()),
                )),
                Box::new(Select::Element(
                    Element {
                        tag: Some(String::from("span")),
                        classes: Vec::new(),
                        id: None,
                        attributes: Vec::new(),
                    },
                    Pseudo::None,
                )),
            ),
            properties: vec![
                Property {
                    name: "font-size",
                    value: PropertyValue::PX(10),
                },
                Property {
                    name: "color",
                    value: PropertyValue::COLOR("#F0F".into()),
                },
            ],
        }],
    };

    println!("{}", stylesheet);
}
