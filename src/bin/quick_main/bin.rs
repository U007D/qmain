#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, /*trivial_casts,*/ trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

#[macro_use]
extern crate quick_main;
main! {() -> Result<()> {
    fn main() {
        extern crate libc;

        #[allow(unused_imports)]
        use ::libc::{EXIT_SUCCESS, EXIT_FAILURE};
        use ::std::io::write;



    }
    foo;
    bar;
    baz;
    when! { baz }
}
}
