#![cfg_attr(target_arch="arm", no_std)]
#![cfg_attr(target_arch="arm", no_main)]

#[cfg(target_arch="arm")]

use core::panic::PanicInfo;

mod resources;
mod diamond;
mod mpe;

use crate::hal::surface::*;
use crate::hal::*;
use crate::mpe::VoiceManager;
use crate::diamond::Diamond;
use crate::resources::TONES;
use launchpad_pro_rs::hal;
use launchpad_pro_rs::hal::LaunchpadApp;
use launchpad_pro_rs::launchpad_app;

/// The Launchpad Pro app state.
struct State {
    /// A flag to indicate whether the Game of Life simulation is running.
    is_running: bool,
    /// JI diamond state
    diamond: Diamond,
    /// MPE voice manager
    mpe: VoiceManager
}

impl State {
    /// Create the app.
    const fn new() -> Self {
        Self {
            is_running: false,
            diamond: Diamond::new(),
            mpe: VoiceManager::new()
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
        //self.life.tick();
    }

    /// Toggle the state of the cell at the point on the grid.
    fn toggle_cell(&mut self, _point: hal::Point) {
        /*
        let toggled_state = !self.life.get(point);
        self.life.set(point, toggled_state);
        */
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    /// Toggle whether the simulation is running.
    fn toggle_is_running(&mut self) {
        self.is_running = ! self.is_running;
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
    fn init_event(&self, _pads: hal::surface::Pads) {
        //let mut state = self.state.lock();
        //state.diamond.update_notes();
        for i in 0..8 {
            for j in 0..8 {
                set_led(Point::new(1 + i as i8, 1 + j as i8), TONES[i][j].rgb())
            }
        }
        let mut state = self.state.lock();
        for i in 0..8 {
            state.mpe.get_voice_mut(i).unwrap().set_channel(i + 1);
        }
    }

    fn timer_event(&self) {
        /// A count of the number of timer callbacks.
        static mut TICKS: i32 = 0;

        unsafe {
            if TICKS == TICKS_PER_FRAME {
                let mut state = self.state.lock();
                if state.is_running() {
                    state.tick();
                    state.draw_universe();
                }
                TICKS = 0;
            } else {
                TICKS += 1;
            }
        }
    }

    fn midi_event(&self, _port: hal::midi::Port, _midi_event: hal::midi::MidiMessage) {
    }

    fn sysex_event(&self, _port: hal::midi::Port, _data: &[u8]) {
    }

    fn cable_event(&self, _cable_event: hal::midi::CableEvent) {
    }

    fn button_event(&self, button_event: hal::surface::ButtonEvent) {
        match button_event.event {
            hal::surface::Event::Press(value) => {
                match button_event.button {
                    surface::Button::Setup => {
                        // Setup pressed
                    },
                    surface::Button::Pad(point) => {
                        if point.x() > 0 && point.x() <= 8 && point.y() > 0 && point.y() <= 8 {
                            let mut state = self.state.lock();
                            match state.mpe.take(point.x() as u8 - 1, point.y() as u8 - 1) {
                                None => {},
                                Some(&voice) => {
                                    // Voice taken
                                    set_led(point, Rgb::new(0xff, 0xff, 0xff))
                                }
                            }
                        }
                    }
                }
            },
            hal::surface::Event::Release => {
                let mut state = self.state.lock();

                match button_event.button {
                    hal::surface::Button::Pad(point) => {
                        if point.x() > 0 && point.x() <= 8 && point.y() > 0 && point.y() <= 8 {
                            match state.mpe.release(point.x() as u8 - 1, point.y() as u8 - 1) {
                                None => {},
                                Some(&mut voice) => {
                                    set_led(point, voice.rgb())
                                }
                            }
                        }
                    }
                    hal::surface::Button::Setup => {
                        state.toggle_is_running();
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
