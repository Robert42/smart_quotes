//! This is a tiny helper crate providing a simple heursitic for
//! implementing smart quotes.
//!
//!
//! While this crate does not convert any glyphs, it gives a heuristic
//! based on the previous character, wheter the next character would be
//! an opening or closing quotation mark.
//!
//! Example usage:
//! ```
//! use smart_quotes::{decide_quote_after, Decision};
//!
//! assert_eq!(decide_quote_after(None), Decision::Open);
//!
//! assert_eq!(decide_quote_after(Some(' ')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\t')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\n')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\x0A')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\u{1680}')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\u{2005}')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\u{202F}')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\u{2029}')), Decision::Open);
//!
//! assert_eq!(decide_quote_after(Some('(')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('[')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('{')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('⟨')), Decision::Open);
//!
//! assert_eq!(decide_quote_after(Some('\u{2012}')), Decision::Open);
//! assert_eq!(decide_quote_after(Some('\u{2015}')), Decision::Open);
//!
//! assert_eq!(decide_quote_after(Some('x')), Decision::Close);
//! assert_eq!(decide_quote_after(Some('“')), Decision::Close);
//! assert_eq!(decide_quote_after(Some('‘')), Decision::Close);
//! assert_eq!(decide_quote_after(Some('.')), Decision::Close);
//! assert_eq!(decide_quote_after(Some(':')), Decision::Close);
//! ```
//!

/// The decision, whether an open or closed quotation mark should be used.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Decision {
    Open,
    Close,
}

/// Applies the heurisic based on the previous_character.
///
/// If the quote would be the first character, you can pass `None`.
pub fn decide_quote_after(previous_character: Option<char>) -> Decision {
    match previous_character {
        None => Decision::Open,
        Some(x) if is_space(x) || is_dash(x) || is_opening_parenthesis(x) => Decision::Open,
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

fn is_dash(x: char) -> bool {
    match x {
        '\u{2012}'..='\u{2015}' => true,
        _ => false,
    }
}

fn is_opening_parenthesis(x: char) -> bool {
    match x {
        '(' | '[' | '{' | '⟨' => true,
        _ => false,
    }
}
