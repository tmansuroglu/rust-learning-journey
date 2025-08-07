//! # chapter_14
//!
//! I'm learning
//! The style of doc comment //! adds documentation to the item that contains the comments rather than to the items following the comments.

/// adds one to the number given.
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = chapter_14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
