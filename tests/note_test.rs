use music_generator::Note;

#[test]
fn note_creation_test() {
    let c_note = Note::new(1, 1);
    assert_eq!(1, c_note.tone)
}