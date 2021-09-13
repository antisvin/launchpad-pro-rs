use super::diamond::*;
use crate::hal::Rgb;
use crate::hal::midi;
use crate::resources::TONES;
use wmidi::{MidiMessage, ControlFunction, Channel, U7, U14};


const MAX_VOICES: usize = 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Voice {
    note: Note,
    row: u8,
    col: u8,
    channel: u8, // Channel 0 == disabled
    is_taken: bool
}

impl Voice {
    const fn new() -> Self {
        Voice { note: Note::empty(), row: 0, col: 0, channel: 0, is_taken: false }
    }

    const fn can_take(&self) -> bool {
        self.channel != 0 && !self.is_taken
    }

    pub fn set_channel(&mut self, channel: u8) -> &mut Self {
        self.channel = channel;
        self
    }

    fn take(&mut self, row: u8, col: u8) -> &mut Self {
        self.is_taken = true;
        self.row = row;
        self.col = col;
        self
    }

    fn release(&mut self) -> &mut Self {
        self.is_taken = false;
        self
    }

    pub fn set_note(&mut self, note: Note) -> &mut Self {
        self.note = note;
        self
    }

    pub const fn rgb(&self) -> Rgb {
        TONES[self.row as usize][self.col as usize].rgb()
    }

    /// Pitch bend sensitivity [RPN 0] - Note level, only integer number of semitones
    /// is supported.
    pub fn pitch_bend_range_message(&self, pitch_bend_range: u8) -> Option<MidiMessage> {
        match self.channel {
            0 => None,
            _ => Some(MidiMessage::ControlChange(
                Channel::from_index(self.channel).unwrap(),
                ControlFunction::NON_REGISTERED_PARAMETER_NUMBER_MSB,
                U7::try_from(pitch_bend_range).unwrap()))
        }
    }

    pub fn send_note_on(&self, velocity: u8) {
        let channel = Channel::from_index(self.channel).unwrap();
        let messages = [
            MidiMessage::PitchBendChange(
                channel, U14::try_from(self.note.pitch_bend()).unwrap()),
            //MidiMessage::ChannelPressure(
            //    channel, U7::try_from(velocity).unwrap()),
            MidiMessage::NoteOn(
                channel, self.note.midi_note(),
                U7::try_from(velocity).unwrap())
        ];
        VoiceManager::send_messages(&messages)
    }

    pub fn send_note_off(&self, velocity: u8) {
        let channel = Channel::from_index(self.channel).unwrap();
        let message = MidiMessage::NoteOff(
            channel, self.note.midi_note(),
            U7::try_from(velocity).unwrap());
        VoiceManager::send_message(&message)
    }
}

enum MPEZone {
    Lower,
    Upper
}

pub struct VoiceManager {
    voices: [Voice; MAX_VOICES],
    num_channels: u8,
    zone: MPEZone,
    num_taken: u8
}

impl VoiceManager {
    pub const fn new() -> Self {
        VoiceManager {
            voices: [Voice::new(); MAX_VOICES], num_channels: MAX_VOICES as u8,
            zone: MPEZone::Lower, num_taken: 0
        }
    }

    pub fn get_voice_mut(&mut self, index: u8) -> Option<&mut Voice>{
        let index: usize = index.into();
        if index < MAX_VOICES {
            Some(&mut self.voices[index])
        }
        else {
            None
        }
    }

    pub fn get_voice(&self, index: u8) -> Option<&Voice>{
        let index: usize = index.into();
        if index < MAX_VOICES {
            Some(&self.voices[index])
        }
        else {
            None
        }
    }

    /// Take a new voice if available
    pub fn take(&mut self, row: u8, col: u8) -> Option<&mut Voice> {
        let mut channels = self.num_channels;
        for v in self.voices.iter_mut() {
            if channels == 0 {
                return None
            }
            if v.can_take() {
                v.take(row, col);
                return Some(v)
            }
            else {
                channels -= 1;
            }
        }
        None
    }

    pub fn release(&mut self, row: u8, col: u8) -> Option<&mut Voice> {
        for v in &mut self.voices.iter_mut() {
            if v.row == row && v.col == col {
                v.release();
                return Some(v)
            }
        };
        None
    }

    /// MPE Configuration Message [RPN 6]
    pub fn mcm_messages(&self) -> [MidiMessage; 3] {
        let channel = match self.zone {
            MPEZone::Lower => Channel::Ch1,
            MPEZone::Upper => Channel::Ch16
        };
        [
            MidiMessage::ControlChange(
                channel, ControlFunction::REGISTERED_PARAMETER_NUMBER_LSB,
                U7::try_from(6).unwrap()
            ),
            MidiMessage::ControlChange(
                channel, ControlFunction::REGISTERED_PARAMETER_NUMBER_MSB,
                U7::try_from(0).unwrap()
            ),
            MidiMessage::ControlChange(
                channel, ControlFunction::DATA_ENTRY_MSB,
                U7::try_from(self.num_channels).unwrap()
            )
        ]
    }

    pub fn mono_mode_message(&self) -> MidiMessage {
        let channel = match self.zone {
            MPEZone::Lower => Channel::Ch1,
            MPEZone::Upper => Channel::Ch16
        };
        MidiMessage::ControlChange(
            channel, ControlFunction::POLY_OPERATION, U7::try_from(1).unwrap()
        )
    }

    pub fn poly_mode_message(&self) -> MidiMessage {
        let channel = match self.zone {
            MPEZone::Lower => Channel::Ch1,
            MPEZone::Upper => Channel::Ch16
        };
        MidiMessage::ControlChange(
            channel, ControlFunction::MONO_OPERATION, U7::try_from(1).unwrap()
        )
    }

    /// Pitch bend sensitivity [RPN 0] - Zone level, only integer number of semitones
    /// is supported.
    pub fn pitch_bend_range_message(&self, pitch_bend_range: u8) -> MidiMessage {
        let channel = match self.zone {
            MPEZone::Lower => Channel::Ch1,
            MPEZone::Upper => Channel::Ch16
        };
        MidiMessage::ControlChange(
            channel, ControlFunction::NON_REGISTERED_PARAMETER_NUMBER_MSB,
            U7::try_from(pitch_bend_range).unwrap()
        )
    }

    fn send_message(msg: &MidiMessage) {
        for port in [midi::Port::USB, midi::Port::DIN] {
            midi::send_message(port, msg)
        }
    }

    fn send_messages(messages: &[MidiMessage]) {
        for msg in messages {
            for port in [midi::Port::USB, midi::Port::DIN] {
                midi::send_message(port, msg)
            }
        }
    }

    pub fn init_mpe(&self, pitch_bend_range: u8) {
        Self::send_messages(&self.mcm_messages());
        Self::send_message(&self.poly_mode_message());
        Self::send_message(&self.pitch_bend_range_message(pitch_bend_range));
    }
}

//pub static mut MPE: VoiceManager = VoiceManager::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager() {
        let mpe = VoiceManager::new();
        assert_eq!(mpe.num_channels, MAX_VOICES as u8);
    }

    #[test]
    fn voice_take_release() {
        let mut v = Voice::new();
        // No channel set - can't take yet
        assert_eq!(v.channel, 0);
        assert_eq!(v.can_take(), false);
        assert_eq!(v.is_taken, false);
        assert_eq!(v.row, 0);
        assert_eq!(v.col, 0);
        // Can take after channel is assigned
        v.set_channel(1);
        assert_eq!(v.can_take(), true);
        v.take(1, 2);
        assert_eq!(v.can_take(), false);
        assert_eq!(v.is_taken, true);
        assert_eq!(v.row, 1);
        assert_eq!(v.col, 2);
        // Release
        v.release();
        assert_eq!(v.can_take(), true);
        assert_eq!(v.is_taken, false);
        assert_eq!(v.row, 1);
        assert_eq!(v.col, 2);
    }

    #[test]
    fn manager_take_no_voices() {
        let mut mpe = VoiceManager::new();
        assert_eq!(mpe.take(1, 2), None);
    }

    #[test]
    fn manager_take_two_voices() {
        let mut mpe = VoiceManager::new();
        // Take first voice
        mpe.get_voice_mut(0).unwrap().set_channel(1);
        let &mut voice1 = mpe.take(1, 2).unwrap();
        assert_eq!(voice1, mpe.voices[0]);
        // Can't take second voice before it has a channel
        assert_eq!(mpe.take(2, 3), None);        
        // Take second voice
        mpe.get_voice_mut(1).unwrap().set_channel(2);
        let &mut voice2 = mpe.take(2, 3).unwrap();
        assert_eq!(voice2, mpe.voices[1]);
        assert_ne!(voice1, voice2);
        // Can't take another one
        assert_eq!(mpe.take(3, 4), None)
    }
}
