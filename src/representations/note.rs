/// A struct representing a single note in music. TODO: Add support for micro-tonal music and switch to a frequency based data structure.
pub struct Note {
    /// The tone of the note. This can be any numerical number from 0 to 11 representing all notes in one octave.
    pub tone: i32,
    /// The octave that the note is in. We will be using 0 to represent the octave that middle C on a piano is in.
    pub octave: i32,
}

/// Implementation for [Note]
impl Note {
    /// Returns a new instance of a note given tone and octave.
    pub fn new(tone: i32, octave: i32) -> Note {
        Note { tone, octave }
    }
}
