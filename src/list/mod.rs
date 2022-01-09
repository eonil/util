// use extend::ext;

// /// A better interface to `im::vector::Vector`.
// #[derive(Clone)]
// #[derive(Eq,PartialEq)]
// #[derive(Ord,PartialOrd)]
// struct List<Item:Clone> {
//     raw: im::vector::Vector<Item>,
// }

// #[ext(name=ListCore)] 
// impl<T:Clone> List<T> {
//     fn is_empty(&self) -> bool {
//         self.raw.is_empty()
//     }
//     fn len(&self) -> usize {
//         self.raw.len()
//     }
//     fn get(&self, i:usize) -> Option<&T> {
//         self.raw.get(i)
//     }
//     fn get_mut(&mut self, i:usize) -> Option<&mut T> {
//         self.raw.get_mut(i)
//     }
//     fn slice(&self, range: std::ops::Range<usize>) -> Self {
//         let c = self.raw.len();
//         let a = range.start == 0;
//         let b = range.end == c;
//         let x = match (a,b) {
//             (true,true) => self.raw,
//             (true,false) => self.raw.take(range.end),
//             (false,true) => self.raw.skip(range.start),
//             (false,false) => self.raw.take(range.end).skip(range.start),
//         };
//         Self { raw: x }
//     }
//     fn push_front(&mut self, x:T) {
//         self.raw.push_front(x)
//     }
//     fn push_back(&mut self, x:T) {
//         self.raw.push_back(x)
//     }
//     fn pop_front(&mut self) {
//         self.raw.pop_front()
//     }
//     fn pop_back(&mut self) {
//         self.raw.pop_back()
//     }
//     fn append(&mut self, x:Self) {
//         self.raw.append(x.raw)
//     }
// }