#![feature(test)]

pub mod common;
pub mod utils;

/// We need this for Rust to detect our testing source files (unittests)
#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod benches;
