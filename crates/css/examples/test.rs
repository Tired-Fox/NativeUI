extern crate cypress_css;

use std::{collections::HashMap, fmt::{Display, Debug}};

use cypress_css::parser::{
    selector::{AttributeSelector, CompoundSelector, Matcher, SelectorCompare},
    stylesheet::StyleSheet,
};

#[derive(Default)]
struct Node {
    name: String,
    id: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{} id=\"{}\" {}>",
            self.name,
            self.id,
            self.attributes
                .iter()
                .map(|(key, value)| format!("{}=\"{}\"", key, value))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl SelectorCompare for Node {
    fn get_id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }

    fn get_tag(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn get_classes(&self) -> Vec<&str> {
        self.attributes
            .get(&String::from("class"))
            .map_or(Vec::new(), |c| c.split(" ").collect::<Vec<&str>>())
    }

    fn get_namespace(&self) -> Option<&str> {
        None
    }

    fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(&key.to_string()).map(|v| v.as_str())
    }
}

fn main() {
    let src = r#"
    #some-id[data-value~='foo' i] > div.some-class#of-pop:first-child:where(p > a, :pseudo-invalid) {
        color: red;
    }
    "#;

    // div#something.green[data-value]
    let selector = CompoundSelector {
        tag: Some("div".into()),
        id: Some("something".into()),
        classes: vec!["green".into()],
        attributes: vec![AttributeSelector {
            name: "data-value".into(),
            matcher: Matcher::Exists,
            expects: None,
            insensitive: false,
        }],
        pseudo_class: None,
        pseudo_element: None,
    };

    // <div id="something" class="green" data-value></div>
    let node = Node {
        name: String::from("div"),
        id: String::from("something"),
        attributes: HashMap::from([
            (String::from("data-value"), String::from("true")),
            (String::from("class"), String::from("green"))
        ]),
        children: Vec::new()
    };

    let invalid_node = Node::default();
    let children = vec![&invalid_node, &node];

    println!("{} == {} = {}", node, selector, node.matches(&selector));
    println!("{} == {} = {}", invalid_node, selector, invalid_node.matches(&selector));
    println!("filtered: {:?}", children.iter().filter(|v| v.matches(&selector)).collect::<Vec<&&Node>>());

    let stylesheet = StyleSheet::source(src);
    println!("{:?}", stylesheet);

    // let stylesheet = StyleSheet::path("examples/styles.css");
    // println!("{:?}", stylesheet);
}
