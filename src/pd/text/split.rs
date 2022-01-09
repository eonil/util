use super::*;

#[ext(name = TextSplitUtil)]
pub impl Text {
    fn first_index_of(&self, x:u8) -> Option<usize> {
        self.tree.index_of(&x)
    }
    fn split_at(self, index:usize) -> (Text,Text) {
        let (a,b) = self.tree.split_at(index);
        (Text { tree: a }, Text { tree: b })
    }
    fn lines(&self) -> TextSplit {
        TextSplit { available: self.clone(), sep: b'\n' }
    }
}

pub struct TextSplit {
    available: Text,
    sep: u8,
}
impl Iterator for TextSplit {
    type Item = Text;
    fn next(&mut self) -> Option<Text> {
        if self.available.is_empty() { return None }
        let i = match self.available.first_index_of(self.sep) {
            None => {
                let mut x = Text::new();
                std::mem::swap(&mut x, &mut self.available);
                return Some(x)
            },
            Some(x) => x,
        };
        let (a,b) = self.available.clone().split_at(i);
        self.available = b.clone().split_at(1).1;
        Some(a)
    }
}




#[test]
fn test_rust_string_lines_newline_inclusion() {
    // It does not include the newline character.
    let a = "A\nB";
    let b = a.lines().collect::<Vec<_>>();
    assert_eq!(b[0], "A");
    assert_eq!(b[1], "B");
}

#[test]
fn test_im_vec_split() {
    let mut a = im::vector::Vector::<i32>::new();
    a.push_back(111);
    a.push_back(222);
    a.push_back(333);
    let (b,c) = a.split_at(1);
    assert_eq!(b.len(), 1);
    assert_eq!(c.len(), 2);
    if b.len() != 1 { return }
    if c.len() != 2 { return }
    assert_eq!(b[0], 111);
    assert_eq!(c[0], 222);
    assert_eq!(c[1], 333);
    let (d,e) = c.split_at(1);
    assert_eq!(d.len(), 1);
    assert_eq!(e.len(), 1);
    if d.len() != 1 { return }
    if e.len() != 1 { return }
    assert_eq!(d[0], 222);
    assert_eq!(e[0], 333);
}

#[test]
fn test_lines_newline_inclusion() {
    let a = Text::from("A\nB");
    let b = a.lines().collect::<Vec<_>>();
    assert_eq!(b.len(), 2);
    assert_eq!(b[0].to_string(), "A");
    assert_eq!(b[1].to_string(), "B");
}