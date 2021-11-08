`rhai-rand` - Package to Generate Random Numbers
===============================================

[![License](https://img.shields.io/crates/l/rhai)](https://github.com/license/rhaiscript/rhai-rand)
[![crates.io](https://img.shields.io/crates/v/rhai-rand?logo=rust)](https://crates.io/crates/rhai-rand/)
[![crates.io](https://img.shields.io/crates/d/rhai-rand?logo=rust)](https://crates.io/crates/rhai-rand/)

[![Rhai logo](https://rhai.rs/book/images/logo/rhai-banner-transparent-colour.svg)](https://rhai.rs)

`rhai-rand` is a [Rhai] package to provide random numbers generation using the [`rand`] crate.

[Rhai] is an embedded scripting language and evaluation engine for Rust that gives a safe and easy way
to add scripting to any application.


Usage
-----

`Cargo.toml`:

```toml
[dependencies]
rhai-rand = "0.1"
```

Rust source:

```rust
use rhai::Engine;
use rhai_rand::RandomPackage;

// Create Rhai scripting engine
let mut engine = Engine::new();

// Create random number package and add the package into the engine
engine.register_global_module(RandomPackage::new().as_shared_module());

// Print 10 random numbers!
for _ in 0..10 {
    let value = engine.eval::<INT>("rand()")?;

    println!("Random number = {}", value);
}

// Create array
let a = [1, 2, 3, 4, 5];

// Shuffle it!
a.shuffle();

// Now the array is shuffled randomly!
print(a);
```


Features
--------

|  Feature  | Description                                     |
| :-------: | ----------------------------------------------- |
|  `float`  | enables random floating-point number generation |
| `shuffle` | enables the `shuffle` method for [Rhai] arrays  |


API
---

The following functions are defined in this package:

|     Function      | Return value |  Feature  | Description                                                      |
| :---------------: | :----------: | :-------: | ---------------------------------------------------------------- |
|     `rand()`      |    `INT`     |           | generates a random number                                        |
|  `rand_float()`   |   `FLOAT`    |  `float`  | generates a random floating-point number between `0.0` and `1.0` |
|   `rand_bool()`   |    `bool`    |           | generates a random boolean                                       |
| `Array.shuffle()` |              | `shuffle` | shuffles the items in the [Rhai] array                           |


[Rhai]: https://rhai.rs
[`rand`]: https://crates.io/crates/rand
