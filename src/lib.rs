use rand::prelude::*;
use rhai::{def_package, plugin::*, INT};

#[cfg(feature = "float")]
use rhai::FLOAT;

#[cfg(feature = "shuffle")]
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

    #[cfg(feature = "shuffle")]
    #[rhai_fn(global)]
    pub fn shuffle(array: &mut Array) {
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
    }
}
