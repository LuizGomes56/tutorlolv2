use wiki::formula::SequencePattern::{self, *};

#[test]
fn classifies_rounded_sequences() {
    assert_eq!(
        SequencePattern::new(&[60.0, 95.0, 130.0, 165.0, 200.0]),
        Linear
    );
    assert_eq!(
        SequencePattern::new(&[0.75, 0.813, 0.875, 0.938, 1.0]),
        Linear
    );
    assert_eq!(
        SequencePattern::new(&[33.33, 50.0, 66.67, 83.33, 100.0]),
        Linear
    );
    assert_eq!(SequencePattern::new(&[10.0, 67.5, 137.5, 220.0]), Quadratic);
}

#[test]
fn rejects_step_tables() {
    // Previously accepted as linear because every difference is below 3
    let steps = [0.02, 0.02, 0.02, 0.03, 0.03, 0.03, 0.04, 0.04, 0.04];
    assert_eq!(SequencePattern::new(&steps), Unknown);
    assert_eq!(SequencePattern::new(&[1.0, 2.0, 4.0, 5.0, 6.0]), Unknown);
}

#[test]
fn two_values_are_inconclusive() {
    assert_eq!(SequencePattern::new(&[1.0, 2.0]), Inconclusive);
}
