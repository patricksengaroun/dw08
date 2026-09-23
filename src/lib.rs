//! # DW08 — Sum to N
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

/// Adds up the whole numbers from 1 through `n`.
///
/// # Examples
///
/// ```
/// use dw08::sum_to;
///
/// assert_eq!(sum_to(5), 15);
/// ```
///
/// ```
/// use dw08::sum_to;
///
/// assert_eq!(sum_to(1), 1);
/// ```
///
/// Adding up nothing gives zero:
///
/// ```
/// use dw08::sum_to;
///
/// assert_eq!(sum_to(0), 0);
/// ```
///
/// ```
/// use dw08::sum_to;
///
/// assert_eq!(sum_to(10), 55);
/// ```
pub fn sum_to(n: u32) -> u32 {
    let mut total = 1;
    for i in 1..n {
        total += i;
    }
    total
}
