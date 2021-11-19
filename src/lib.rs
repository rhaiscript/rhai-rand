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
