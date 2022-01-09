
/// Represents a persistent datastructure collection modeled after Swift standard library.
/// 
/// I was skeptical about need for this kind of library at first,
/// but realized that immutable persistent datastructure can benefit Rust ecosystem
/// side-by-side with existing mutable ephemiral datastructures.
trait Collection {
    type Iter: Iterator;
    fn iter(&self) -> Self::Iter;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
}