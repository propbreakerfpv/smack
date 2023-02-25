use super::css::*;

const LENGTH_UNITS: &str = "px";

pub fn parce(source: String) -> StyleSheet {
    StyleSheet{
        rules: Parcer {
            pos: 0,
            content: source,
        }.parse_rules()
    }
}

struct Parcer {
    pos: usize,
    content: String,
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false,
    }
}


impl Parcer {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            if self.end() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        let selectors = self.parse_selectors();
        let declarations = self.parse_declarations();
        return Rule {
            selectors,
            declarations,
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn consume_char_if(&mut self, eq: char) -> Option<char> {
        if self.next_char() == eq {
            Some(self.consume_char())
        } else {
            None
        }
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.consume_while(|c| valid_identifier_char(c));
        self.consume_char_if(':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_char_if(';');

        Declaration {
            name,
            value,
        }
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255 })
    }

    /// Parse two hexadecimal digits.
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.content[self.pos .. self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    fn parse_length(&mut self) -> Value {
        let num = self.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        let unit = match self.consume_while(|c| {LENGTH_UNITS.contains(c)}).as_str() {
            "px" => Unit::Px,
            _ => panic!("unhandeled length unit"),
        };
        Value::Length(num.parse::<f32>().unwrap(), unit)
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '#' => { // color
                self.parse_color()
            }
            '0'..='9' | '.' => { // length
                self.parse_length()
            }
            _ => self.parse_keyword(),
        }
    }

    fn parse_keyword(&mut self) -> Value {
        Value::Keyward(self.consume_while(|c| valid_identifier_char(c)))
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            self.consume_whitespace();
            selectors.push(Selector::Simple(self.parse_simpl_selector()));
            self.consume_whitespace();
            if self.next_char() == '{' || self.end() {
                break;
            }
        }
        selectors
    }

    fn parse_simpl_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector::new();
        while !self.end() {
            match self.next_char() {
                '#' => { // id
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => { // class
                    selector.class.push(self.parse_identifier());
                }
                '*' => { // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => { // for tag names. ie `div { display: none}`
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(|c| valid_identifier_char(c))
    }
    
    fn consume_while<F>(&mut self, f: F) -> String 
    where F: Fn(char) -> bool {
        let mut out = String::new();
        while ! self.end() && f(self.next_char()) {
            out.push(self.consume_char());
        }
        out
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.content[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|char| char.is_whitespace())
    }

    fn end(&self) -> bool {
        self.pos >= self.content.len() - 1
    }

    fn next_char(&self) -> char {
        self.content[self.pos..].chars().next().unwrap()
    }

}
