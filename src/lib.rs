#![crate_name = "music_generator"]

pub mod representations;
pub use representations::note::Note;
pub use representations::interval::Interval;

/// There are 12 simi-tone notes in an octave.
static NOTES_PER_OCTAVE: i32 = 12;
