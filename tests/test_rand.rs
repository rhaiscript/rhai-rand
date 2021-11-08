use rhai::{packages::Package, Engine, EvalAltResult, INT};
use rhai_rand::RandomPackage;

#[cfg(feature = "float")]
use rhai::FLOAT;

#[cfg(feature = "shuffle")]
use rhai::Array;

#[test]
fn test_rand() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let first = engine.eval::<INT>("rand()")?;
    let second = engine.eval::<INT>("rand()")?;

    assert!(first != second);

    #[cfg(feature = "float")]
    {
        let first = engine.eval::<FLOAT>("rand_float()")?;
        let second = engine.eval::<FLOAT>("rand_float()")?;

        assert!(first != second);
    }

    for _ in 0..10 {
        let value = engine.eval::<INT>("(rand() % 100).abs()")?;

        println!("Random number = {}", value);
    }

    Ok(())
}

#[cfg(feature = "shuffle")]
#[test]
fn test_shuffle() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let array = engine.eval::<Array>(
        "
            let a = [1, 2, 3, 4, 5];
            a.shuffle();
            a
        ",
    )?;

    println!("Array = {:?}", array);

    let array: Vec<_> = array.into_iter().map(|v| v.as_int().unwrap()).collect();

    assert_ne!(array, vec![1 as INT, 2, 3, 4, 5]);

    Ok(())
}
