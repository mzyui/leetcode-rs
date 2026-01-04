//! Analysis-only crate root.
//!
//! This file exists solely to enable IDE tooling (rust-analyzer).
//! It includes all solution files for analysis purposes only.
//!
//! DO NOT use this crate for building or running code.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod solutions {
    include!("../solutions/1.two-sum.rs");
    include!("../solutions/2.add-two-numbers.rs");
    include!("../solutions/3.longest-substring-without-repeating-characters.rs");
    include!("../solutions/4.median-of-two-sorted-arrays.rs");
}
