# smart_quotes

## Smart Quotes

This is a tiny helper crate providing a simple heursitic for
implementing smart quotes.


While this crate does not convert any glyphs, it gives a heuristic
based on the previous character, wheter the next character would be
an opening or closing quotation mark.

Example usage:
```rust
use smart_quotes::{decide_quote_after, Decision};

assert_eq!(decide_quote_after(None), Decision::Open);

assert_eq!(decide_quote_after(Some(' ')), Decision::Open);
assert_eq!(decide_quote_after(Some('\t')), Decision::Open);
assert_eq!(decide_quote_after(Some('\n')), Decision::Open);
assert_eq!(decide_quote_after(Some('\x0A')), Decision::Open);
assert_eq!(decide_quote_after(Some('\u{1680}')), Decision::Open);
assert_eq!(decide_quote_after(Some('\u{2005}')), Decision::Open);
assert_eq!(decide_quote_after(Some('\u{202F}')), Decision::Open);
assert_eq!(decide_quote_after(Some('\u{2029}')), Decision::Open);

assert_eq!(decide_quote_after(Some('(')), Decision::Open);
assert_eq!(decide_quote_after(Some('[')), Decision::Open);
assert_eq!(decide_quote_after(Some('{')), Decision::Open);
assert_eq!(decide_quote_after(Some('⟨')), Decision::Open);

assert_eq!(decide_quote_after(Some('\u{2012}')), Decision::Open);
assert_eq!(decide_quote_after(Some('\u{2015}')), Decision::Open);

assert_eq!(decide_quote_after(Some('x')), Decision::Close);
assert_eq!(decide_quote_after(Some('“')), Decision::Close);
assert_eq!(decide_quote_after(Some('‘')), Decision::Close);
assert_eq!(decide_quote_after(Some('.')), Decision::Close);
assert_eq!(decide_quote_after(Some(':')), Decision::Close);
```


License: MIT OR Apache-2.0
