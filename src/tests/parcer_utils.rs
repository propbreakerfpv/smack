use crate::renderer::parcer_utils::strings::HTMLStringExt;
use crate::renderer::parcer_utils::strings::HTMLString;
use crate::renderer::parcer_utils;

#[test]
fn parse_intager() {
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("-156")), Some(-156));
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("156")), Some(156));
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("+156")), Some(156));
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("2147483647")), Some(2147483647));
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("2147483648")), None);
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("-2147483647")), Some(-2147483647));
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("-2147483648")), None);
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("")), None);
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("-")), None);
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("+")), None);
    assert_eq!(parcer_utils::numbers::parse_intager(String::from("+1")), Some(1));
}
#[test]
fn starts_with_whitespace() {
    assert_eq!(HTMLString::from(" jksl").starts_with_whitespace(), false);
    assert_eq!(HTMLString::from("\tjksl").starts_with_whitespace(), true);
    assert_eq!(HTMLString::from("\rjksl").starts_with_whitespace(), true);
    assert_eq!(HTMLString::from("\njksl").starts_with_whitespace(), true);
    assert_eq!(HTMLString::from("jksl").starts_with_whitespace(), false);
    assert_eq!(HTMLString::from("").starts_with_whitespace(), false);
    assert_eq!(HTMLString::from("\r").starts_with_whitespace(), true);
}

#[test]
fn trim_start() {
    assert_eq!(HTMLString::from("\r").trim_start(), HTMLString::from(""));
    assert_eq!(HTMLString::from("\rtest").trim_start(), HTMLString::from("test"));
    assert_eq!(HTMLString::from("\ttest").trim_start(), HTMLString::from("test"));
    assert_eq!(HTMLString::from("\ntest").trim_start(), HTMLString::from("test"));
    assert_eq!(HTMLString::from("test").trim_start(), HTMLString::from("test"));
    assert_eq!(HTMLString::from("").trim_start(), HTMLString::from(""));
}
