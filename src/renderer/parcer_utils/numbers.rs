use super::strings::HTMLString;
use super::strings::HTMLStringExt;

/// takes a string and returns its intager representation acording to the HTML standard
/// # TODO
/// should return a Result<i32, some error type>
/// # note
/// this may not be complyent as it asumes the intager is of type i32. the spec does not specify
/// the size so this is just a guess
pub fn parse_intager(mut input: HTMLString) -> Option<i32> {
    let mut position = 0;
    let mut is_positive = true;
    input = input.trim_start();
    if position >= input.len() {
        return None;
    }
    // if the first char is a - then the sign is negative
    if input.as_bytes()[0] == 45 {
        is_positive = false;
        position += 1;
        if position >= input.len() {
            return None;
        }
    }
    // if the first char is a + then just skip it 
    if input.as_bytes()[0] == 43 {
        position += 1;
        if position >= input.len() {
            return None;
        }
    }
    let mut iter = input[position..].char_indices();
    let mut value = String::new();
    loop {
        match iter.next() {
            Some((_, c)) => {
                value.push(c)
            }
            None => {
                if is_positive {
                    return match value.parse::<i32>() {
                        Ok(v) => Some(v),
                        Err(_) => return None,
                    };
                } else {
                    return match value.parse::<i32>() {
                        Ok(v) => Some(0 - v),
                        Err(_) => return None,
                    };
                }
            },
        };
    }
}

