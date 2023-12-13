# Selectors

## Simple Selector:

```css
div {}
```

## Compound Selecter

```css
div#id.class {}
```

## Complex Selector

```css
div > .class ~ #id {
}
```

## Relative Selector

All selectors are relative selectors with an implied descendant combinator as the default
```css
+ div#id > #reference {}
> .icon {}
dt:has(+ img) ~ dd {}
```

## Selector List

comma seperated list of simple, compound, and/or complex selectors. An element matches the selector list if it matches
at least on of the selectors. If a Non-Forgiving selector is invalid then the entire list is invalid.
`:is()` and `:where()` pseudo classes create forgiving selector lists.

## Combinators

+ `*` | ` `: Descendant Combinator; selects nodes that are descendant of the first element.
+ `>`: Child Combinator; selects nodes that are direct children of the first element.
+ `~`: Subsequent-sibling Combinator; given `A ~ B`, all eleemnts matching `B` will be selected if they are preceded by `A`.
+ `+`: Next-sibling Combinator: given `A + B`, it only matches `B` if `B` is immediately preceded by `A`, with both sharing the same parent.
+ `||`: Column Combinator: selects nodes that belong to a column.
  + Ex: `col || td` will match all `<td>` elements that belong to the scope of the `<col>`
+ `|`: Namespace Combinator: enables limiting type selectors and universal selectors to a specific namespace.
  + EX: `@namespace SVG url('http://www.w3.org/2000/svg')` would allow for `SVG | a {}` and this will only apply to `a` tags inside of an SVG.

## Pseudo Element

Keyword added to a selector that lets you style a specific part of the selected element(s)

## Pseudo Function/Class

Keyword added to a selector that specifies a special state of the selected element(s). Ex: `:hover` can be used to select a button when a user's pointer hovers over the button

# Structure

- Relative Selector
  - Complex Selector
    - Combinator
    - Compound Selector
      - Type Selector / Universal Selector
      - Id Selector
      - Class Selector
      - Attribute Selector
      - Pseudo Selector
```rust

enum Combinator{}
enum Matcher{}
enum PseudoClass{}
enum PseudoElement{}
struct Attribute {
    name: &str,
    matcher: Matcher,
    needle: &str
}
struct CompundSelector {
  tag: Option<&str>,
  id: Option<&str>,
  classes: Vec<&str>,
  attributes: Vec<Attribute>,
  pseudo_class: Option<PseudoClass>,
  pseudo_element: Option<PseudoElement>,
}
enum ComplexSelector {
  Combinator(Combinator),
  CompoundSelector(CompoundSelector)
}
struct RelativeSelector(pub Vec<ComplexSelector>);

struct SelectorList(pub Vec<RelativeSelector>);
```