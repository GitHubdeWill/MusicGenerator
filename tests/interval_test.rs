use music_generator::{Interval, Note};

#[test]
fn interval_creation_test() {
    let test_interval: Interval = Interval::new(4);
    assert_eq!(4, test_interval.raw_distance);
}

#[test]
fn interval_calculation_same_octave_test() {
    let from: Note = Note::new(1, 4);
    let to: Note = Note::new(3, 4);
    let test_interval: Interval = Interval::between(from, to);
    assert_eq!(2, test_interval.raw_distance);
}


#[test]
fn interval_calculation_diff_octave_test() {
    let from: Note = Note::new(3, 3);
    let to: Note = Note::new(1, 4);
    let test_interval: Interval = Interval::between(from, to);
    assert_eq!(10, test_interval.raw_distance);
}