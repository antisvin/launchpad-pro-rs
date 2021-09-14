#![cfg_attr(target_arch="arm", no_std)]
#![cfg_attr(target_arch="arm", no_main)]

#[cfg(target_arch="arm")]

use core::panic::PanicInfo;

mod resources;
mod diamond;
mod mpe;

use crate::hal::surface::*;
use crate::hal::*;
use crate::mpe::{VoiceManager, MAX_VOICES};
use crate::diamond::Diamond;
use crate::resources::TONES;
use launchpad_pro_rs::hal;
use launchpad_pro_rs::hal::LaunchpadApp;
use launchpad_pro_rs::launchpad_app;

/// The Launchpad Pro app state.
struct State {
    /// JI diamond state
    diamond: Diamond,
    /// MPE voice manager
    mpe: VoiceManager,
    pads: Option<Pads>,
    init_delay: u8
}

const DEFAULT_INIT_DELAY: u8 = 100;

impl State {
    /// Create the app.
    const fn new() -> Self {
        Self {
            diamond: Diamond::new(),
            mpe: VoiceManager::new(),
            pads: None,
            init_delay: DEFAULT_INIT_DELAY
        }
    }

    /// Draw the Game of Life universe on the Launchpad Pro grid.
    fn draw_universe(&self) {
        /*
        for point in hal::Grid::points() {
            hal::surface::set_led(
                point,
                match self.life.get(point) {
                    life::Cell::Alive => hal::Rgb::new(0, 255, 0),
                    life::Cell::Dead => hal::Rgb::new(0, 0, 0),
                },
            );
        }
        */
    }

    /// Move the simulation forward by one tick.
    fn tick(&mut self) {
        match self.init_delay {
            0 => (),
            1 => {
                self.mpe.init_mpe(self.diamond.pitch_bend_range());
                self.init_delay = 0;
            },
            _ => self.init_delay -= 1
        }
    }

    /// Toggle the state of the cell at the point on the grid.
    fn toggle_cell(&mut self, _point: hal::Point) {
        /*
        let toggled_state = !self.life.get(point);
        self.life.set(point, toggled_state);
        */
    }

    fn schedule_init(&mut self) {
        self.init_delay = DEFAULT_INIT_DELAY;
    }
}

struct App {
    state: hal::Mutex<State>
}

impl App {
    const fn new() -> Self {
        Self {
            state: hal::Mutex::new(State::new())
        }
    }
}

/// The number of frames per second in our simulation.
const FRAMES_PER_SECOND: i32 = 4;
/// The number of timer ticks per frame. Timer ticks happen at a frequency of ~1ms.
const TICKS_PER_FRAME: i32 = 1000 / FRAMES_PER_SECOND;

#[derive(Clone, Copy)]
pub enum Colour {
    Black = 0x0f0f0f,
    Red = 0xff0000,
	Orange = 0xffa500,
	Yellow = 0xffff00,
	Green = 0x008000,
	Blue = 0x0000ff,
	Purple = 0x4b0082,
	Magenta = 0xee82ee
}

use crate::Colour::*;

pub const COLOURS: [Colour; 16] = [
    Black, Black, Black, Red, Black, Orange, Black, Yellow, Black, Green,
    Black, Blue, Black, Purple, Black, Magenta
];

/// Implement the LaunchpadApp trait for our app in order to be notified of events that occur on
/// the Launchpad Pro hardware.
impl LaunchpadApp for App {
    fn init_event(&self, pads: hal::surface::Pads) {
        for i in 0..8 {
            for j in 0..8 {
                set_led(Point::new(1 + i as i8, 1 + j as i8), TONES[i][j].rgb())
            }
        }
        let mut state = self.state.lock();
        state.diamond.update_notes();
        let mut channels: [u8; MAX_VOICES] = [0; MAX_VOICES];
        for i in 0..MAX_VOICES {
            channels[i] = i as u8 + 1
        }
        state.mpe.fill_voices(&channels);
        for i in 0..MAX_VOICES {
            state.mpe.get_voice_mut(i as u8).unwrap().set_channel(i as u8 + 1);
        }
        state.pads = Some(pads)
    }

    fn timer_event(&self) {
        let mut state = self.state.lock();
        state.tick();
        //state.draw_universe();
    }

    fn midi_event(&self, _port: hal::midi::Port, _midi_event: hal::midi::MidiMessage) {
    }

    fn sysex_event(&self, _port: hal::midi::Port, _data: &[u8]) {
    }

    fn cable_event(&self, cable_event: hal::midi::CableEvent) {
        if let hal::midi::CableEvent::Connect(_cable) = cable_event {
            let mut state = self.state.lock();
            state.schedule_init()
        }
    }

    fn button_event(&self, button_event: hal::surface::ButtonEvent) {
        match button_event.event {
            hal::surface::Event::Press(value) => {
                match button_event.button {
                    surface::Button::Setup => {
                        // Setup pressed
                    },
                    surface::Button::Pad(point) => {
                        let row = point.x() as u8 - 1;
                        let col = point.y() as u8 - 1;
                        if row < 8 && col < 8 {
                            let mut state = self.state.lock();
                            let note = state.diamond.get_note(row as usize, col as usize);
                            if let Some(voice) = &mut state.mpe.take(row, col) {
                                use crate::midi::Port;
                                use wmidi::{MidiMessage, ControlFunction, U7, Channel};
                                // Voice taken
                                set_led(point, Rgb::new(0xff, 0xff, 0xff));
                                voice.set_note(note);
                                voice.send_note_on(value);
                            }
                        }
                    }
                }
            },
            hal::surface::Event::Release => {
                let mut state = self.state.lock();

                match button_event.button {
                    hal::surface::Button::Pad(point) => {
                        let row = point.x() as u8 - 1;
                        let col = point.y() as u8 - 1;
                        if row < 8 && col < 8 {
                            if let Some(&mut voice) = state.mpe.release(row, col) {
                                set_led(point, voice.rgb());
                                voice.send_note_off(0 as u8);
                            }
                        }
                    }
                    hal::surface::Button::Setup => {
                        //state.toggle_is_running();
                    }
                }
            }
        }
    }

    fn aftertouch_event(&self, _aftertouch_event: hal::surface::AftertouchEvent) {
    }

}

/// Create a static instance of our app.
static APP: App = App::new();

// Register our app to receive events from the hardware.
launchpad_app!(APP);

#[cfg(target_arch="arm")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_arch="arm"))]
fn main() {}

#[cfg(test)]
mod tests {
    //use super::*;

}
