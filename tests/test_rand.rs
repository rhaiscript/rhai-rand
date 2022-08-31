use rhai::{packages::Package, Engine, EvalAltResult, INT};
use rhai_rand::RandomPackage;

#[cfg(feature = "float")]
use rhai::FLOAT;

#[cfg(feature = "decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "array")]
use rhai::Array;

#[test]
fn test_rand() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let first = engine.eval::<INT>("rand()")?;
    let second = engine.eval::<INT>("rand()")?;

    assert!(first != second);

    let _first = engine.eval::<bool>("rand_bool()")?;

    #[cfg(feature = "float")]
    {
        let first = engine.eval::<FLOAT>("rand_float()")?;
        let second = engine.eval::<FLOAT>("rand_float()")?;

        assert!(first != second);

        let _first = engine.eval::<bool>("rand_bool(0.01)")?;
    }

    #[cfg(feature = "decimal")]
    {
        let first = engine.eval::<Decimal>("rand_decimal()")?;
        let second = engine.eval::<Decimal>("rand_decimal()")?;

        assert!(first != second);

        let first =
            engine.eval::<Decimal>("rand_decimal(123.456.to_decimal(), 789.234.to_decimal())")?;
        let second =
            engine.eval::<Decimal>("rand_decimal(123.456.to_decimal(), 789.234.to_decimal())")?;

        assert!(first != second);
    }

    for _ in 0..10 {
        let value = engine.eval::<INT>("(rand() % 100).abs()")?;

        println!("Random number = {}", value);
    }

    #[cfg(feature = "float")]
    for _ in 0..10 {
        let value = engine.eval::<FLOAT>("rand_float()")?;

        println!("Random float = {}", value);
    }

    Ok(())
}

#[cfg(feature = "array")]
#[test]
fn test_sample() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    assert_eq!(
        engine.eval::<bool>(
            "
                let x = ['a', 'b', 'c', 'd'];
                let s = x.sample();
                x.index_of(s) != -1
            "
        )?,
        true,
        "Should return a random element from the array"
    );

    assert_eq!(
        engine.eval::<()>(
            "
                let x = [];
                x.sample()
            "
        )?,
        (),
        "Should handle empty arrays"
    );

    Ok(())
}

#[cfg(feature = "array")]
#[test]
fn test_sample2() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let array = engine.eval::<Array>(
        "
            let a = ['a', 'b', 'c', 'd'];
            a.sample(3)
        ",
    )?;
    assert_eq!(
        array.len(),
        3,
        "Should return an array matching the requested sample size"
    );

    assert_eq!(
        engine.eval::<bool>(
            "
                let a = ['a', 'b', 'c', 'd'];
                let b = a.sample(4);
                b.sort();
                a == b
            "
        )?,
        true,
        "Should not return any duplicate samples"
    );

    let array = engine.eval::<Array>(
        "
            let a = ['a', 'b', 'c', 'd'];
            a.sample(5)
        ",
    )?;
    assert_eq!(array.len(), 4, "Should be limited to the array's size");

    let array = engine.eval::<Array>(
        "
            let a = ['a', 'b', 'c', 'd'];
            a.sample(0)
        ",
    )?;
    assert_eq!(array.len(), 0, "Zero should return an empty array");

    let array = engine.eval::<Array>(
        "
            let a = ['a', 'b', 'c', 'd'];
            a.sample(-1)
        ",
    )?;
    assert_eq!(
        array.len(),
        0,
        "Negative amounts should return an empty array"
    );

    Ok(())
}

#[cfg(feature = "array")]
#[test]
fn test_shuffle() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let array = engine.eval::<Array>(
        "
            let a = [];
            a.shuffle();
            a
        ",
    )?;

    assert_eq!(array.len(), 0, "Should not affect empty arrays");

    let array = engine.eval::<Array>(
        "
            // 1/10^12 chance this will shuffle into the original order
            let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
            a.shuffle();
            a
        ",
    )?;

    println!("Array = {:?}", array);

    let array: Vec<_> = array.into_iter().map(|v| v.as_int().unwrap()).collect();

    assert_eq!(array.len(), 15, "Array length should not change");

    for n in 1..15 {
        array
            .iter()
            .position(|&v| v == n)
            .expect(format!("Number {} was lost in the shuffle", n).as_str());
    }

    assert_ne!(
        array,
        vec![1 as INT, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        "Array should no longer be sorted"
    );

    Ok(())
}
