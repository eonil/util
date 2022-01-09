mod conversion;
mod split;
mod join;
mod ops;
mod code;


use extend::ext;
use unicode_segmentation::UnicodeSegmentation;
use split::*;
use join::*;

type U8Tree = im::vector::Vector<u8>;

#[derive(Default)]
#[derive(Eq,PartialEq)]
#[derive(Ord,PartialOrd)]
#[derive(Clone)]
pub struct Text {
    tree: U8Tree,
}
impl Text {
    pub fn new() -> Text { Text { tree: U8Tree::new() } }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn bytes<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a u8> {
        self.tree.leaves().flatten()
    }
    pub fn pieces<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a [u8]> {
        self.tree.leaves()
    }
    pub fn graphemes<'a>(&'a self) -> impl DoubleEndedIterator<Item = TextGrapheme<'a>> {
        self.tree.leaves().map(u8chunk_to_graphemes).flatten().map(TextGrapheme::from)
    }
    pub fn chars<'a>(&'a self) -> impl DoubleEndedIterator<Item = char> + 'a {
        self.graphemes().map(|g| g.as_str().chars()).flatten()
    }
    pub fn to_lowercase(&self) -> Text {
        // Keep this single-alloc, zero-copy.
        let mut x = String::new();
        for leaf in self.tree.leaves() {
            let s = unsafe { std::str::from_utf8_unchecked(leaf) };
            for ch in s.chars() {
                x.extend(ch.to_lowercase());
            }
        }
        Text { tree: U8Tree::from(x.into_bytes()) }
    }
}
fn u8chunk_to_graphemes<'a>(units: &'a [u8]) -> unicode_segmentation::Graphemes<'a> {
    unsafe { std::str::from_utf8_unchecked(units) }.graphemes(true)
}


pub struct TextGrapheme<'a> {
    imp: &'a str,
}
impl<'a> TextGrapheme<'a> {
    pub fn as_str(&self) -> &'a str {
        self.imp
    }
    pub fn chars(&self) -> impl DoubleEndedIterator<Item = char> + 'a {
        self.as_str().chars()
    }
    pub fn is_ascii(&self) -> bool {
        self.imp.chars().nth(0).unwrap().is_ascii()
    }
    pub fn is_digit(&self, radix: u32) -> bool {
        self.imp.chars().nth(0).unwrap().is_digit(radix)
    }
    pub fn is_uppercase(&self) -> bool {
        self.imp.chars().nth(0).unwrap().is_uppercase()
    }
    pub fn is_lowercase(&self) -> bool {
        self.imp.chars().nth(0).unwrap().is_lowercase()
    }
    // pub fn is_newline(&self) -> bool {
    //     // Force-unwrap as `TextGrapheme` cannot be empty.
    //     let last = self.imp.chars().next_back().unwrap();
    //     last == '\n'
    // }
}















