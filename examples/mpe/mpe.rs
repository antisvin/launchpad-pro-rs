use super::diamond::*;

const MAX_VOICES: usize = 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Voice {
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

    fn set_channel(&mut self, channel: u8) -> &mut Self {
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

    /// Take a new voice if available
    pub fn take(&mut self, row: u8, col: u8) -> Option<&Voice> {
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

    pub fn release(&mut self, row: u8, col: u8) -> bool {
        for v in &mut self.voices.iter_mut() {
            if v.row == row && v.col == col {
                v.release();
                return true
            }
        }
        false
    }
}

//pub static mut MPE: VoiceManager = VoiceManager::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager() {
        let mut mpe = VoiceManager::new();
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
        mpe.get_voice_mut(0).unwrap().set_channel(1);
        let &voice1 = mpe.take(1, 2).unwrap();
        assert_eq!(voice1, mpe.voices[0]);
        assert_eq!(mpe.take(2, 3), None);
        mpe.get_voice_mut(1).unwrap().set_channel(2);
        let &voice2 = mpe.take(2, 3).unwrap();
        assert_eq!(voice2, mpe.voices[1]);
        assert_ne!(voice1, voice2)
    }
}
