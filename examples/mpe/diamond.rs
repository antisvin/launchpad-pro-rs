//use crate::hal::{Grid, Point};
//use super::diamond::resources;
use super::COLOURS;
use crate::hal::Rgb;
use crate::resources::TONES;
use wmidi::Note as MidiNote;

// Number of tones in diamond row/column
const DIAMOND_SIZE: usize = 8;

/// Precomputed JI tone
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tone {
    limit: u8,
    semitones: u8,
    cents: u32,
    rgb: Rgb,
}

impl Tone {
    /// Create new tone
    pub const fn new(limit: u8, semitones: u8, cents: u32) -> Self {
        let col = COLOURS[limit as usize] as u32;
        let r: u8 = (col >> 16) as u8;
        let g: u8 = ((col >> 8) & 0xff) as u8;
        let b: u8 = (col & 0xff) as u8;
        Tone {
            limit,
            semitones,
            cents,
            rgb: Rgb::new(r, g, b),
        }
    }
    pub const fn limit(&self) -> u8 {
        self.limit
    }
    pub const fn rgb(&self) -> Rgb {
        self.rgb
    }
}

/// Microtonal note for MIDI - can be changed at runtime
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    midi_note: MidiNote,
    bend: u16,
}

impl Note {
    /// New note
    pub const fn empty() -> Self {
        Note {
            midi_note: MidiNote::CMinus1,
            bend: 0,
        }
    }
    /// New note with number/pitch bend assigned
    pub const fn new(midi_note: MidiNote, bend: u16) -> Self {
        Note { midi_note, bend }
    }
    /// Current pitch bend value
    pub const fn pitch_bend(&self) -> u16 {
        self.bend
    }
    /// MIDI note
    pub const fn midi_note(&self) -> MidiNote {
        self.midi_note
    }
}

type NotesGrid = [[Note; DIAMOND_SIZE]; DIAMOND_SIZE];

/// 8 x 8 15-limit JI tonal diamond
pub struct Diamond {
    base_note: u8,
    pitch_bend_range: u8,
    notes: NotesGrid,
}

impl Diamond {
    /// Create a new diamond
    pub const fn new() -> Self {
        Diamond {
            base_note: 24,
            pitch_bend_range: 1,
            notes: [[Note::empty(); DIAMOND_SIZE]; DIAMOND_SIZE],
        }
    }

    /// Set base note (MIDI note number)
    fn set_base_note(&mut self, note: u8) {
        self.base_note = note;
        self.update_notes()
    }

    /// Get pitch bend range
    pub const fn pitch_bend_range(&self) -> u8 {
        self.pitch_bend_range
    }

    /// Set pitch bend range in semitones. The bigger the less precise.
    fn set_pitch_bend_range(&mut self, range: u8) {
        self.pitch_bend_range = range;
        self.update_notes()
    }

    /// Update note - called when base note or pitch bend range is changed
    pub fn update_notes(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                let mut note = &mut self.notes[i][j];
                note.midi_note = MidiNote::from_u8_lossy(self.base_note + TONES[i][j].semitones);
                note.bend =
                    (TONES[i][j].cents / ((self.pitch_bend_range as u32) << 19)) as u16 + 8192
            }
        }
    }

    /// Get note by row and column number
    pub const fn get_note(&self, row: usize, col: usize) -> Note {
        return self.notes[row][col];
    }
}

#[cfg(test)]
mod tests {
    use crate::diamond::*;
    use crate::resources::TONES;

    #[test]
    fn test_tone_semitones() {
        assert_eq!(TONES[0][0].semitones, 0);
        assert_eq!(TONES[0][4].semitones, 7)
    }

    impl Note {
        fn to_cents(&self, bend_range: u32) -> f64 {
            (self.bend - 8192) as f64 / 8192.0 * (bend_range as f64)
        }
    }

    #[test]
    fn test_diamond() {
        let mut d = Diamond::new();
        d.update_notes();
        assert_eq!(d.base_note, 24 as u8);
        assert_eq!(d.pitch_bend_range, 1);
        // Initial notes check
        assert_eq!(d.get_note(0, 0), Note::new(MidiNote::C1, 8192));
        assert_eq!(d.get_note(0, 1), Note::new(MidiNote::D1, 8512));
        assert_eq!(d.get_note(0, 2), Note::new(MidiNote::Eb1, 15262));
        assert_eq!(d.get_note(0, 4), Note::new(MidiNote::G1, 8352));
        d.set_base_note(36);
        // Go up one octave - only semitones amount changes
        assert_eq!(d.get_note(0, 0), Note::new(MidiNote::C2, 8192));
        assert_eq!(d.get_note(0, 1), Note::new(MidiNote::D2, 8512));
        assert_eq!(d.get_note(0, 2), Note::new(MidiNote::Eb2, 15262));
        assert_eq!(d.get_note(0, 4), Note::new(MidiNote::G2, 8352));

        assert_eq!(d.get_note(0, 0).to_cents(1), 0.0);
        assert_eq!(d.get_note(0, 1).to_cents(1), 0.0390625);
        assert_eq!(d.get_note(0, 2).to_cents(1), 0.863037109375);
        assert_eq!(d.get_note(0, 4).to_cents(1), 0.01953125);
        assert_eq!(d.get_note(0, 4).midi_note(), MidiNote::G2);

        // Larger bend range means that lower numbers are used for bend
        d.set_pitch_bend_range(48);
        assert_eq!(d.get_note(0, 0), Note::new(MidiNote::C2, 8192));
        assert_eq!(d.get_note(0, 1), Note::new(MidiNote::D2, 8198));
        assert_eq!(d.get_note(0, 2), Note::new(MidiNote::Eb2, 8339));
        assert_eq!(d.get_note(0, 4), Note::new(MidiNote::G2, 8195));

        // This results in some precision loss
        assert_eq!(d.get_note(0, 0).to_cents(48), 0.0);
        assert_eq!(d.get_note(0, 1).to_cents(48), 0.03515625);
        assert_eq!(d.get_note(0, 2).to_cents(48), 0.861328125);
        assert_eq!(d.get_note(0, 4).to_cents(48), 0.017578125);
    }
}
