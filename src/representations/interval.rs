use crate::{NOTES_PER_OCTAVE, Note};


/// A struct that represents distance between two [`Note`]
pub struct Interval {
    /// raw distance between two notes
    pub raw_distance: i32,
}

/// Implementation for [Interval]
impl Interval {

    pub fn new(raw_distance: i32) -> Interval {
        Interval {
            raw_distance
        }
    }
    pub fn between(from: Note, to: Note) -> Interval {
        Interval {
            raw_distance: to.tone - from.tone + NOTES_PER_OCTAVE * (to.octave - from.octave),
        }
    }
}