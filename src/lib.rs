pub enum Decision {
    Open,
    Close,
}

pub fn decide_quote_after(previous_character: Option<char>) -> Decision {
    match previous_character {
        None => Decision::Open,
        Some(x) if is_space(x) || is_opening_parenthesis(x) => Decision::Open,
        _ => Decision::Close,
    }
}

fn is_space(x: char) -> bool {
    match x {
        // whitespace (space and linebreaks)
        // https://en.wikipedia.org/w/index.php?title=Whitespace_character&oldid=924029247
        '\x09'..='\x0d'
        | '\x20'
        | '\u{85}'
        | '\u{a0}'
        | '\u{1680}'
        | '\u{2000}'..='\u{200A}'
        | '\u{2028}'
        | '\u{2029}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}' => true,
        '\u{200B}' => true, // zero width space added here, because it indicates the separation between words
        _ => false,
    }
}

fn is_opening_parenthesis(x: char) -> bool {
    match x {
        '(' | '[' | '{' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
