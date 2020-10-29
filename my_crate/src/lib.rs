// THIS IS OUTER DOC COMMENTS
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

// THIS IS INNER DOC COMMENTS
/// Adds one to the number given. Dank memes.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
///
/// # Errors
///
/// # Safety
///
///
///

pub fn add_one(x: i32) -> i32 {
    x + 1
}
