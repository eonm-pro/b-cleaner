#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![allow(dead_code)]

//! # B-cleaner (Bibliographical data cleaner)
//!
//! [![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)]()
//! [![Dependabot Status](https://badgen.net/dependabot/eonm-abes/b-cleaner?icon=dependabot)]()
//! 
//! B-cleaner is a Rust library dedicated to bibliographical data preprocessing (simplification, normalization). This library is used for preprocessing data in alignement tasks. B-cleaner is designed to have a small memory footprint and high performances.
//! 
//! B-cleaner offers **binding with Python 3**.
//! To compile b-cleaner as a Python library make sure you are building this library with the python features enabled : `cargo build --release --lib --features=python`. You can also use [maturin](https://github.com/PyO3/maturin).
//! 
//! ## Usage
//!  
//! B-cleaner works with tokenized data. Tokenized data should contain punctuation.
//! 
//! ### Rust usage
//! 
//! ```
//! use b_cleaner::{TitleCleaner, Clean};
//!
//! fn main() {
//!     let raw_data : Vec<&str> = "Lorem ipsum dolor : sit amet".split_whitespace().collect();
//!     let mut title = TitleCleaner::new(&raw_data);
//!     
//!     title.clean();
//!     
//!     assert_eq!(title.tokens(), &vec!["lorem", "ipsum", "dolor"]);
//! }
//! ```
//! 
//! ## Features :
//! 
//! * **stem** : Add stemming capabilities
//! * **python** : Add bindings with python
//! * **html** : Add HTML transformation capabilities
//! 

mod cleaners;
pub use cleaners::*;

mod bindings;

#[cfg(feature = "python")]
use bindings::python;
