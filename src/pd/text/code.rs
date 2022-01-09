use super::*;

#[ext(name=TextCodeUtil)]
pub impl Text {
    fn camel_to_snake_case(&self) -> Text {
        // Keep this single alloc, zero-copy.
        if self.is_empty() { return Text::new() }
        use crate::iter::IteratorUtil;
        use crate::iter::DoubleEndedIteratorUtil;
        let a = self.graphemes().drop_last();
        let b = self.graphemes().drop_first();
        let c = Iterator::zip(a,b);
        let mut s = String::new();
        // Add first grapheme lowercased.
        let chs = self.graphemes().nth(0).unwrap().chars().map(char::to_lowercase).flatten();
        s.extend(chs);
        for (a,b) in c {
            if a.is_lowercase() && b.is_uppercase() {
                // Boundary. Insert underbar.
                s.push('_');
            }
            s.extend(b.chars().map(char::to_lowercase).flatten());
        }
        Text::from(s)
    }
    fn indent(&self) -> Text {
        let mut z = Text::new();
        for line in self.lines() {
            z.extend_str("    ");
            z.extend_text(line);
            z.extend_str("\n");
        }
        z
    }
}






#[test]
fn test_camel_to_snake_case() {
    let a = "greenAppleAndGrape";
    let b = Text::from(a);
    let c = b.camel_to_snake_case();
    assert_eq!(c.to_string().as_str(), "green_apple_and_grape");
}

#[test]
fn test_evil_case_1() {
    let a = "";
    let b = Text::from(a);
    let c = b.camel_to_snake_case();
    assert_eq!(c.to_string().as_str(), "");
}
#[test]
fn test_evil_case_2() {
    let a = "A";
    let b = Text::from(a);
    let c = b.camel_to_snake_case();
    assert_eq!(c.to_string().as_str(), "a");
}
#[test]
fn test_evil_case_3() {
    let a = "ABC";
    let b = Text::from(a);
    let c = b.camel_to_snake_case();
    assert_eq!(c.to_string().as_str(), "abc");
}
#[test]
fn test_evil_case_4() {
    let a = "a";
    let b = Text::from(a);
    let c = b.camel_to_snake_case();
    assert_eq!(c.to_string().as_str(), "a");
}
#[test]
fn test_evil_case_5() {
    let a = "aB";
    let b = Text::from(a);
    let c = b.camel_to_snake_case();
    assert_eq!(c.to_string().as_str(), "a_b");
}