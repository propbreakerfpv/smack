use super::dom::*;

pub fn parce(source: String) -> Node {
    Node::from_elem("html".to_string(), AttrMap::new(), Parcer{
        pos: 0,
        content: source
    }.parce_nodes())
}

pub struct Parcer {
    pos: usize,
    content: String,
}

impl Parcer {

    fn parce_nodes(&mut self) -> Nodes {
        let mut nodes = Nodes::new();
        loop {
            self.consume_whitespace();

            if self.end() || self.starts_with("</") {
                break;
            }
            nodes.nodes.push(self.parce_node());
        }
        nodes
    }

    fn parce_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    fn parse_text(&mut self) -> Node {
        Node::from_text(self.consume_while(|c| c != '<'))
    }

    fn consume_char_if(&mut self, eq: char) -> Option<char> {
        if self.next_char() == eq {
            Some(self.consume_char())
        } else {
            None
        }
    }

    fn parse_element(&mut self) -> Node {
        assert_eq!(self.consume_char(), '<');
        if self.starts_with("!--") {
            return Node::from_comment(self.consume_while(|c|c == '>'));
        }
        let name = self.parse_name();
        let attrs = self.parse_attrs();
        self.consume_char_if('/');
        assert_eq!(self.consume_char(), '>');

        let children = self.parce_nodes();

        self.consume_whitespace();
        if self.next_char() == '<' {
            self.consume_char();
        }
        self.consume_while(|c| c != '>');

        self.consume_char();
        Node::from_elem(name, attrs, children)
    }

    fn parse_attrs(&mut self) -> AttrMap {
        let mut atters = AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' || self.next_char() == '/' || self.end() {
                break;
            }
            let name = self.parse_name();
            if self.next_char() == '=' {
                self.consume_char();
            }
            if self.next_char() == '"' {
                self.consume_char();
            }
            let value = self.consume_next_attr();
            if self.next_char() == '"' {
                self.consume_char();
            }
            atters.attrs.insert(name, value);
        }
        atters
    }

    fn consume_next_attr(&mut self) -> String {
        self.consume_while(|c| match c {
            '"' => false,
            _ => true,
        })
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
            _ => false,
        })
    }

    fn starts_with(&self, pat: &str) -> bool {
        self.content[self.pos..].starts_with(pat)
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

    fn next_char(&self) -> char {
        self.content[self.pos..].chars().next().unwrap()
    }

    fn end(&self) -> bool {
        self.pos >= self.content.len() - 1
    }
}


