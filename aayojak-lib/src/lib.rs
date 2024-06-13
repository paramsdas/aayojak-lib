//! aayojak-lib
//!
//! Aayojak is a tool which should help organize day to day tasks. This crate is a library crate and contains the basic data structures used by Aayojak.

pub mod structures;
pub mod traits;
pub use self::structures::todo::Todo;
