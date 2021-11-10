use rand::prelude::*;
use rhai::{def_package, plugin::*, INT};

#[cfg(feature = "float")]
use rhai::FLOAT;

#[cfg(feature = "array_functions")]
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

    #[cfg(feature = "array_functions")]
    #[rhai_fn(global)]
    pub fn sample(array: &mut Array) -> rhai::Dynamic {
        if !array.is_empty() {
            let mut rng = rand::thread_rng();
            if let Some(res) = array.choose(&mut rng) {
                return Dynamic::from(res.clone())
            }
        }
        Dynamic::from(())
    }

    #[cfg(feature = "array_functions")]
    #[rhai_fn(global)]
    pub fn shuffle(array: &mut Array) {
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
    }
}
