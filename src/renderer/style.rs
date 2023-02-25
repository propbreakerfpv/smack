use std::collections::HashMap;
use crate::renderer::dom::Node;
use crate::renderer::css::Value;

use super::css::{Selector, SimpleSelector, Rule};
use super::dom::ElementData;

struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>
}

type PropertyMap = HashMap<String, Value>;


fn matches_selector(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|n| elem.tag_name != *n) {
        return false;
    }
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }
    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
        return false;
    }

    return true;
}


fn match_rule(elem: &ElementData, rule: &Rule) -> Option<u32> {
    let matches = rule.selectors.iter().filter(|selector| {
        matches_selector(elem, selector)
    });

    Some(1)
}


