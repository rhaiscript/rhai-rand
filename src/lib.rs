//! # `rhai-rand` - Rhai Functions for Random Number Generation
//!
//! `rhai-rand` is a [Rhai] package to provide random number generation using the [`rand`] crate.
//!
//!
//! ## Usage
//!
//!
//! ### `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! rhai-rand = "0.1"
//! ```
//!
//! ### [Rhai] script
//!
//! ```js
//! // Create random boolean
//! let decision = rand_bool();
//!
//! if decision {
//!     // Create random number
//!     let random_value = rand();
//!     print(`Random number = ${random_value}`);
//! } else {
//!     print("Fixed number = 42");
//! }
//!
//! // Create array
//! let a = [1, 2, 3, 4, 5];
//!
//! // Shuffle it!
//! a.shuffle();
//!
//! // Now the array is shuffled randomly!
//! print(a);
//!
//! // Sample a random value from the array
//! print(a.sample());
//!
//! // Or sample multiple values
//! print(a.sample(3));
//! ```
//!
//! ### Rust source
//!
//! ```rust
//! # fn main() -> Result<(), Box<rhai::EvalAltResult>> {
//! use rhai::Engine;
//! use rhai::packages::Package;
//!
//! use rhai_rand::RandomPackage;
//!
//! // Create Rhai scripting engine
//! let mut engine = Engine::new();
//!
//! // Create random number package and add the package into the engine
//! engine.register_global_module(RandomPackage::new().as_shared_module());
//!
//! // Print 10 random numbers, each of which between 0-100!
//! for _ in 0..10 {
//!     let value = engine.eval::<i64>("rand(0..=100)")?;
//!
//!     println!("Random number = {}", value);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! |  Feature   | Default  | Description                                                                        |
//! | :--------: | :------: | ---------------------------------------------------------------------------------- |
//! | `metadata` | disabled | includes functions metadata: parameter names/types, return type, doc-comments      |
//! | `decimal`  | disabled | provides random [decimal](https://crates.io/crates/rust_decimal) number generation |
//! |  `float`   | enabled  | provides random floating-point number generation                                   |
//! |  `array`   | enabled  | provides methods for [Rhai] arrays                                                 |
//!
//!
//! ## API
//!
//! The following functions are defined in this package:
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-rand-docs.md"))]
//!
//! [Rhai]: https://rhai.rs
//! [`rand`]: https://crates.io/crates/rand
//! [`Decimal`]: https://crates.io/crates/rust_decimal

use rhai::def_package;
use rhai::plugin::*;

#[cfg(feature = "array")]
mod array;
mod rand;

def_package! {
    /// Package for random number generation, sampling and shuffling.
    pub RandomPackage(lib) {
        combine_with_exported_module!(lib, "rand", rand::rand_functions);

        #[cfg(feature = "array")]
        combine_with_exported_module!(lib, "array", array::array_functions);
    }
}
