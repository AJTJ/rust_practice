use std::borrow::Cow;

pub mod cell;
pub mod rc;
pub mod refcell;

// the COW type
// Basically, if you don't need to modify then you don't need to allocate memory, and the COW type allows you to do that.
// fn escape(s: &'a str) -> Cow<'a, str> {
//     // ' => \'
//     // ' => \"
//     // foo => foo
//     if no_editing_needed(s) {
//         Cow::Borrowed(s)
//     } else {
//         let mut string = s.to_string();
//         // do something to string (like add \)
//         Cow::Owned(string)
//     }
// }
