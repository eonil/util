use super::*;

#[ext(name=TextJoinUtil)]
pub impl Text {
    fn extend_text(&mut self, x:Self) {
        self.tree.extend(x.tree)
    }
    fn extend_str(&mut self, x:&str) {
        self.tree.extend(x.bytes())
    }
}

#[test]
fn test_extend_text() {
    let mut a = Text::from("A");
    let b = Text::from("B");
    a.extend_text(b);
    assert_eq!(a.to_string().as_str(), "AB");
}
#[test]
fn test_extend_str() {
    let mut a = Text::from("A");
    a.extend_str("B");
    assert_eq!(a.to_string().as_str(), "AB");
}