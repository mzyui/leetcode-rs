//! Analysis-only crate root.
//!
//! This file exists solely to enable IDE tooling (rust-analyzer).
//! It includes all solution files for analysis purposes only.
//!
//! DO NOT use this crate for building or running code.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

mod solutions {
    include!("../solutions/1.two-sum.rs");
    include!("../solutions/2.add-two-numbers.rs");
}
