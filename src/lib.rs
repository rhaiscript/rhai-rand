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
//! // packages::Package implements `as_shared_module`
//! // which we need to register the RandomPackage
//! use rhai::{Engine, packages::Package};
//! use rhai_rand::RandomPackage;
//!
//! // Create Rhai scripting engine
//! let mut engine = Engine::new();
//!
//! // Create random number package and add the package into the engine
//! engine.register_global_module(RandomPackage::new().as_shared_module());
//!
//! // Print 10 random numbers, each of which between 0-99!
//! for _ in 0..10 {
//!     let value = engine.eval::<INT>("(rand() % 100).abs()")?;
//!
//!     println!("Random number = {}", value);
//! }
//! ```
//!
//!
//! ## Features
//!
//!
//! | Feature | Default | Description                                      |
//! | :-----: | :-----: | ------------------------------------------------ |
//! | `float` | Enabled | provides random floating-point number generation |
//! | `array` | Enabled | provides methods for [Rhai] arrays               |
//!
//!
//! ## API
//!
//!
//! The following functions are defined in this package:
//!
//! |      Function      | Return value | Feature | Description                                                            |
//! | :----------------: | :----------: | :-----: | ---------------------------------------------------------------------- |
//! |      `rand()`      |    `INT`     |         | generates a random number                                              |
//! |   `rand_float()`   |   `FLOAT`    | `float` | generates a random floating-point number between `0.0` and `1.0`       |
//! |   `rand_bool()`    |    `bool`    |         | generates a random boolean                                             |
//! | `Array.shuffle()`  |              | `array` | shuffles the items in the [Rhai] array                                 |
//! |  `Array.sample()`  |  `Dynamic`   | `array` | copies a random element from the [Rhai] array                          |
//! |  `Array.sample(n)` |   `Array`    | `array` | copies a non-repeating random sample of elements from the [Rhai] array |
//!
//!
//!
//! [Rhai]: https://rhai.rs
//! [`rand`]: https://crates.io/crates/rand

use rand::prelude::*;
use rhai::{def_package, plugin::*, INT};

#[cfg(feature = "float")]
use rhai::FLOAT;

#[cfg(feature = "array")]
use rhai::Array;

def_package!(rhai:RandomPackage:"Random number generation.", lib, {
    combine_with_exported_module!(lib, "rand", rand_functions);
});

#[export_module]
mod rand_functions {
    pub fn rand_bool() -> bool {
        rand::random()
    }

    pub fn rand() -> INT {
        rand::random()
    }

    #[cfg(feature = "float")]
    pub fn rand_float() -> FLOAT {
        rand::random()
    }

    #[cfg(feature = "array")]
    #[rhai_fn(global)]
    pub fn sample(array: &mut Array) -> rhai::Dynamic {
        if !array.is_empty() {
            let mut rng = rand::thread_rng();
            if let Some(res) = array.choose(&mut rng) {
                return res.clone()
            }
        }
        Dynamic::UNIT
    }

    #[cfg(feature = "array")]
    #[rhai_fn(global, name = "sample")]
    pub fn sample2(array: &mut Array, amount: rhai::INT) -> Array {
        if array.is_empty() || amount <= 0 {
            return Array::new()
        }

        let mut rng = rand::thread_rng();
        let amount = amount as usize;

        if amount >= array.len() {
            let mut res = array.clone();
            res.shuffle(&mut rng);
            res
        } else {
            let mut res: Array = array.choose_multiple(&mut rng, amount).cloned().collect();
            // Although the elements are selected randomly, the order of elements in
            // the buffer is neither stable nor fully random. So we must shuffle the
            // result to achieve random ordering.
            res.shuffle(&mut rng);
            res
        }
    }

    #[cfg(feature = "array")]
    #[rhai_fn(global)]
    pub fn shuffle(array: &mut Array) {
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
    }
}
