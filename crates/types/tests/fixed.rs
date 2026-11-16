use types::{CritDamage, Percent};

#[test]
fn fixed_rounds_to_the_nearest_step() {
    for value in [0.0, 1.0, 12.5, 35.2, 99.99, 100.0] {
        assert!(
            (Percent::new(value).get() - value).abs() <= 0.5 / 512.0,
            "{value}"
        );
    }

    for value in [175.0, 200.0, 231.45, 300.0] {
        assert!(
            (CritDamage::new(value).get() - value).abs() <= 0.5 / 128.0,
            "{value}"
        );
    }
}

#[test]
fn fixed_saturates_out_of_range() {
    assert_eq!(Percent::new(-10.0).get(), 0.0);
    assert_eq!(Percent::new(f32::NAN).get(), 0.0);
    assert_eq!(Percent::new(1e9).get(), Percent::MAX);
    const { assert!(Percent::MAX > 127.99 && CritDamage::MAX > 511.99) };
}

#[test]
fn fixed_is_two_bytes_and_converts_to_f32() {
    const { assert!(size_of::<Percent>() == 2) };
    assert_eq!(f32::from(Percent::from(50.0)), 50.0);
}
