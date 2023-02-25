
pub type HTMLString = String;

pub trait HTMLStringExt{
    fn starts_with_whitespace(&self) -> bool;
    fn trim_start(&self) -> HTMLString;
}


impl HTMLStringExt for HTMLString {
    /// returns wether the given string starts with whitespace 
    /// according to the infra spec 
    /// https://infra.spec.whatwg.org/#ascii-whitespace
    fn starts_with_whitespace(&self) -> bool {
        if self.len() == 0 {
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





