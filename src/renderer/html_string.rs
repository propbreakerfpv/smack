use std::fmt::Display;

pub struct HTMLString {
    vec: Vec<char>
}

impl HTMLString {
    pub fn new() -> HTMLString {
        HTMLString {
            vec: Vec::new(),
        }
    }

    pub fn from(s: &str) -> HTMLString {

        HTMLString {
            vec: s.to_string().chars().collect(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.vec.push(c);
    }

    /// # NOT TO spec
    /// #### this could be out of spec if the string us utf16 not utf8
    pub fn as_bytes(&self) -> Vec<u8> {
        let x: Vec<[u8; 4]> = self.vec.iter().map(|c| {
            let utf8_bytes = [0; 4];
            c.encode_utf8(&mut utf8_bytes).as_bytes();
            utf8_bytes
        }).collect();
        let out = Vec::new();
        x.iter().for_each(|e| {
            e.iter().for_each(|byte| out.push(*byte))
        });
        out
    }

    pub fn slice(&self, start: usize, end: usize) -> HTMLString {
        
    }
    /// returns wether the given string starts with whitespace 
    /// according to the infra spec 
    /// https://infra.spec.whatwg.org/#ascii-whitespace
    fn starts_with_whitespace(&self) -> bool {
        if self.vec.len() == 0 {
            return false;
        }
        if self.as_bytes()[0] == 9 {
            return true;
        }else if self.as_bytes()[0] == 10 {
            return true;
        }else if self.as_bytes()[0] == 13 {
            return true;
        }
        return false;
    }

    /// removes whitespace at the begining of the string
    /// according to the infra spec 
    /// https://infra.spec.whatwg.org/#ascii-whitespace
    fn trim_start(&self) -> HTMLString {
        let mut pos = 0;
        loop {
            if pos >= self[pos..].len() {
                return self[pos..].to_string();
            }
            if ! &self[pos..].to_string().starts_with_whitespace() {
                return self[pos..].to_string();
            }
            pos += 1;
        }
    }
}

impl Display for HTMLString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.vec.iter().collect::<String>())
    }
}


pub trait ToHTMLString: ToString {
    fn to_html_string(&self) -> HTMLString {
        HTMLString {
            vec: self.to_string().chars().collect()
        }
    }
}

impl ToHTMLString for String {
    fn to_html_string(&self) -> HTMLString {
        HTMLString {
            vec: self.chars().collect(),
        }
    }
}

