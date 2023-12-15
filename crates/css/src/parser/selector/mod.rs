mod attribute;
mod combinator;
mod compound;
mod pseudo;

use crate::parser::selector::combinator::Combinator;
use crate::parser::stylesheet::StyleParseError;
use crate::parser::Parse;
use cssparser::{Delimiter, ParseError, ParseErrorKind, Parser, Token, ParserInput};
use std::{fmt::{Debug, Display, Formatter}, slice::Iter, vec::IntoIter, iter::StepBy, cmp::{min, max}};

pub use attribute::{AttributeSelector, Matcher};
pub use compound::CompoundSelector;
pub use pseudo::{Direction, Nth, Parity, PseudoClass, PseudoElement};

#[derive(Debug, PartialEq, Clone)]
pub enum ComplexSelector<'i> {
    Combinator(Combinator),
    Compound(CompoundSelector<'i>),
}

impl<'i> Display for ComplexSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ComplexSelector::Combinator(combinator) => combinator.to_string(),
                ComplexSelector::Compound(compound) => compound.to_string(),
            }
        )
    }
}

/// Element and Combinator parts of a selector
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelativeSelector<'i> {
    parts: Vec<ComplexSelector<'i>>,
}

impl<'i> From<&'i str> for RelativeSelector<'i> {
    fn from(value: &'i str) -> Self {
        let mut input = ParserInput::new(value);
        let mut parser = Parser::new(&mut input);
        RelativeSelector::parse(&mut parser).unwrap_or(RelativeSelector::default())
    }
}

impl<'i> From<&'i String> for RelativeSelector<'i> {
    fn from(value: &'i String) -> Self {
        RelativeSelector::from(value.as_str())
    }
}

impl<'i> Display for RelativeSelector<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.parts.iter().peekable();
        if let Some(ComplexSelector::Combinator(Combinator::Descendant)) = iter.peek() {
            iter.next();
        }

        write!(
            f,
            "{}",
            iter.map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl<'i> RelativeSelector<'i> {
    pub fn add_part(&mut self, part: ComplexSelector<'i>) {
        self.parts.push(part);
    }
}

enum Previous {
    Combinator,
    Compound,
}
impl<'i, 't> Parse<'i, 't> for RelativeSelector<'i> {
    fn parse(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut selector = RelativeSelector::default();

        let mut prev = Previous::Compound;
        loop {
            if input.is_exhausted() {
                break;
            }

            input.skip_whitespace();
            // Combinator or Compound. Cannot have two combinators in a row.
            // Compound after Compound assumes Descendant Combinator; inject it.
            let before = input.state();
            match Combinator::parse(input) {
                Ok(combinator) => {
                    if let Previous::Combinator = prev {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(StyleParseError::UnexpectedCombinator),
                            location: input.current_source_location(),
                        });
                    }
                    selector.add_part(ComplexSelector::Combinator(combinator));
                    prev = Previous::Combinator;
                }
                Err(..) => {
                    input.reset(&before);
                    match CompoundSelector::parse(input) {
                        Ok(result) => {
                            if let Previous::Compound = prev {
                                selector
                                    .add_part(ComplexSelector::Combinator(Combinator::Descendant));
                            }
                            selector.add_part(ComplexSelector::Compound(result));
                            prev = Previous::Compound;
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
            }
        }

        Ok(selector)
    }
}

/// Comman seperated list of selectors
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SelectorList<'i> {
    pub(crate) pattern: Vec<RelativeSelector<'i>>,
}

impl<'i> Display for SelectorList<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pattern
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<'i> SelectorList<'i> {
    pub fn add_selector(&mut self, selector: RelativeSelector<'i>) {
        self.pattern.push(selector);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RelativeSelector<'i>> {
        self.pattern.iter()
    }
}

impl<'i, 't> SelectorList<'i> {
    pub fn parse(
        input: &mut Parser<'i, 't>,
        forgiving: bool,
    ) -> Result<Self, ParseError<'i, StyleParseError>> {
        let mut list = SelectorList::default();
        loop {
            let selector =
                input.parse_until_before(Delimiter::Comma, |i| RelativeSelector::parse(i));

            let ok = selector.is_ok();
            match selector {
                Ok(selector) => {
                    list.add_selector(selector);
                }
                Err(err) => {
                    if !forgiving {
                        return Err(err);
                    }
                }
            }

            loop {
                match input.next() {
                    Err(_) => return Ok(list),
                    Ok(&Token::Comma) => break,
                    Ok(_) => {
                        debug_assert!(!ok, "Shouldn't get a selector if there was an error");
                    }
                }
            }
        }
    }
}

fn cmp_exists<T: PartialEq>(exists: Option<T>, other: Option<T>) -> bool {
    if exists.is_some() {
        return exists == other
    }
    true
}

pub trait SelectorCompare {
    // Get Name
    fn get_tag(&self) -> Option<&str>;
    // Get Id
    fn get_id(&self) -> Option<&str>;
    // Get Classes
    fn get_classes(&self) -> Vec<&str>;
    // Get Attributes
    fn get_attribute(&self, key: &str) -> Option<&str>;
    // Get Namespace
    fn get_namespace(&self) -> Option<&str>;

    fn matches(&self, other: &CompoundSelector) -> bool {
        let uclasses = self.get_classes();
        cmp_exists(other.tag.as_deref(), self.get_tag())
            && cmp_exists(other.id.as_deref(), self.get_id())
            && other.classes.iter().all(|c| uclasses.contains(&c.as_ref()))
            && other.attributes.iter().all(
                |AttributeSelector {
                     name,
                     matcher,
                     expects,
                     insensitive,
                 }| {
                    let attr = self.get_attribute(name.as_ref());
                    matcher.matches(expects.as_ref(), attr, *insensitive)
                },
            )
    }
}

pub fn get_nth<'i, N: SelectorCompare, T: IntoIterator<Item = &'i N>>(source: T, nth: &Nth) -> Vec<&'i N> {
    match nth {
        Nth::Parity(Parity::Odd) => source.into_iter().skip(1).step_by(2).collect::<Vec<&N>>(),
        Nth::Parity(Parity::Even) => source.into_iter().step_by(2).collect::<Vec<&N>>(),
        Nth::Functional { step, offset, of } => {
            if let Some(of) = of.as_ref() {
                if *step > 0 {
                    source.into_iter().filter(|v| of.matches(*v)).skip(max(0, *offset-1)).step_by(*step as usize).collect::<Vec<&N>>()
                } else {
                    source.into_iter().filter(|v| of.matches(*v)).take(*offset).collect::<Vec<&N>>()
                }
            } else {

                if *step > 0 {
                    source.into_iter().skip(max(0, *offset-1)).step_by(*step as usize).collect::<Vec<&N>>()
                } else {
                    source.into_iter().take(*offset).collect::<Vec<&N>>()
                }
            }
            
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use cssparser::{ParserInput, Parser};

    use crate::parser::{selector::{ComplexSelector, combinator::Combinator, CompoundSelector, SelectorList}, Parse};

    use super::{RelativeSelector, get_nth, SelectorCompare, Nth, Parity};

    #[test]
    fn relative_from_string() {
        let expected = RelativeSelector { parts: vec![
            ComplexSelector::Combinator(Combinator::Descendant),
            ComplexSelector::Compound(CompoundSelector { tag: Some("p".into()), ..Default::default() }),
            ComplexSelector::Combinator(Combinator::Child),
            ComplexSelector::Compound(CompoundSelector { tag: Some("a".into()), ..Default::default() }),
        ] };

        let slice = RelativeSelector::from("p > a");
        assert!(slice == expected);
    
        let selector = String::from("p > a");
        let string = RelativeSelector::from(&selector);
        assert!(string == expected);
    }

    #[test]
    fn parse_relative_selector() {
        let expected = RelativeSelector { parts: vec![
            ComplexSelector::Combinator(Combinator::Descendant),
            ComplexSelector::Compound(CompoundSelector { tag: Some("p".into()), ..Default::default() }),
            ComplexSelector::Combinator(Combinator::Child),
            ComplexSelector::Compound(CompoundSelector { tag: Some("a".into()), ..Default::default() }),
        ] };

        let src = "p > a";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = RelativeSelector::parse(&mut parser);
        assert!(result.is_ok());
        assert!(result.unwrap() == expected);
    }

    #[test]
    fn parse_forgiving_selector_list() {
        let expected = RelativeSelector { parts: vec![
            ComplexSelector::Combinator(Combinator::Descendant),
            ComplexSelector::Compound(CompoundSelector { tag: Some("p".into()), ..Default::default() }),
            ComplexSelector::Combinator(Combinator::Child),
            ComplexSelector::Compound(CompoundSelector { tag: Some("a".into()), ..Default::default() }),
        ] };

        let src = "p > a , :pseudo-invalid";
        let mut input = ParserInput::new(src);
        let mut parser = Parser::new(&mut input);
        let result = SelectorList::parse(&mut parser, true);
        assert!(result.is_ok());
        assert!(result.unwrap() == SelectorList { pattern: vec![expected] });
    }

    #[derive(Clone, Default, Debug)]
    struct Node {
        name: Option<String>,
        id: Option<String>,
        attributes: HashMap<String, String>,
    }

    impl Node {
        fn new(name: Option<&str>, id: Option<&str>, attributes: &[(&'static str, &'static str)]) -> Self {
            Self {
                name: name.map(|v| v.to_string()),
                id: id.map(|v| v.to_string()),
                attributes: {
                    let mut map = HashMap::new();
                    map.extend(
                        attributes.iter().map(|(k, v)| (k.to_string(), v.to_string()))
                    );
                    map
                }
            }
        }
    }

    impl SelectorCompare for Node {
        fn get_id(&self) -> Option<&str> {
            self.id.as_ref().map(|v| v.as_str())
        }
    
        fn get_tag(&self) -> Option<&str> {
            self.name.as_ref().map(|v| v.as_str())
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

    #[test]
    fn get_nth_elements() {
        let source = vec![
            Node::new(Some("p"), Some("0"), &[]),
            Node::new(None, Some("1"), &[]),
            Node::new(Some("div"), Some("2"), &[]),
            Node::new(Some("a"), Some("3"), &[]),
            Node::new(Some("div"), Some("4"), &[]),
            Node::new(Some("button"), Some("5"), &[("onclick", "alert(\"Hello, world!\")")]),
            Node::new(None, Some("6"), &[]),
            Node::new(Some("section"), Some("7"), &[("aria-label", "Main Content")]),
            Node::new(Some(""), Some("8"), &[]),
            Node::new(Some("div"), Some("9"), &[]),
            Node::new(Some("div"), Some("10"), &[]),
        ];

        let odd = Nth::Parity(Parity::Odd);
        let even = Nth::Parity(Parity::Even);
        let custom = Nth::Functional { step: 3, offset: 2, of: None };
        let custom_of_div = Nth::Functional { step: 2, offset: 1, of: Some(CompoundSelector { tag: Some("div".into()), ..Default::default() }) };
        let first_three = Nth::Functional { step: -1, offset: 3, of: Some(CompoundSelector { tag: Some("div".into()), ..Default::default() }) };

        let odds = get_nth(&source, &odd);
        assert!(odds.len() == 5);
        let evens = get_nth(&source, &even);
        assert!(evens.len() == 6);
        let customs = get_nth(&source, &custom);
        assert!(customs.len() == 4);
        let custom_ofs = get_nth(&source, &custom_of_div);
        assert!(custom_ofs.len() == 2);
        let first_threes = get_nth(&source, &first_three);
        assert!(first_threes.len() == 3);
    }
}
