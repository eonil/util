use super::*;

impl std::ops::Add<Text> for Text {
    type Output = Text;
    fn add(self, x:Text) -> Text {
        let mut z = self.clone();
        z.extend_text(x);
        z
    }
}