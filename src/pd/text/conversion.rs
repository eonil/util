use super::*;

impl From<&[u8]> for Text {
    fn from(x: &[u8]) -> Self {
        Text { tree: U8Tree::from(x) }
    }
}
impl From<Vec<u8>> for Text {
    fn from(x: Vec<u8>) -> Self {
        Text { tree: U8Tree::from(x) }
    }
}
impl From<String> for Text {
    fn from(x: String) -> Self {
        Text::from(x.into_bytes())
    }
}
impl From<&str> for Text {
    fn from(x: &str) -> Self {
        Text { tree: U8Tree::from(x.as_bytes()) }
    }
}
impl<'a> From<TextGrapheme<'a>> for Text {
    fn from(x: TextGrapheme<'a>) -> Self {
        // Copy contents.
        Text { tree: U8Tree::from(x.as_str().as_bytes()) } 
    }
}
impl std::string::ToString for Text {
    fn to_string(&self) -> String {
        self.chars().collect::<String>()
    }
}



impl<'a> From<&'a str> for TextGrapheme<'a> {
    fn from(s: &'a str) -> TextGrapheme<'a> {
        if s.is_empty() { panic!("empty string cannot built a Unicode Grapheme Cluster") }
        TextGrapheme { imp: s }
    }
}