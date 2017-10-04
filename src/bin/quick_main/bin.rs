#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, /*trivial_casts,*/ trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate quick_main;
extern crate libc;

use quick_main::{StdError, Args, ErrorExt};
#[allow(unused_imports)]
use libc::{EXIT_SUCCESS, EXIT_FAILURE};

pub fn main() {
    match quick_main::run(Args::from(std::env::args_os())) {
        Ok(r) => r,
        Err(ref e) => {
            println!("{}", (e as &StdError).trace());
            std::process::exit(EXIT_FAILURE);
        },
    }
}
