#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

#[cfg(test)]
mod unit_tests;
#[macro_export]
macro_rules! main {
    (() -> Result<$r:ty> { $( $blk:stmt ),*; when! { $( $blk2:stmt),* }}) => (fn main() { println!("main() -> Result<{}> {}", stringify!($r), stringify!($blk));});

}
