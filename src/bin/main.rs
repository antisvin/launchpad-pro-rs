#![cfg_attr(target_arch="arm", no_std)]
#![cfg_attr(target_arch="arm", no_main)]

#[cfg(target_arch="arm")]
use core::panic::PanicInfo;
use wmidi::MidiMessage;
use launchpad_pro_rs::hal;
use launchpad_pro_rs::hal::LaunchpadApp;
use launchpad_pro_rs::launchpad_app;

/// The Launchpad Pro app.
struct App;

// Register an app instance to receive events from the hardware.
launchpad_app!(App);

/// Implementation of the EventListener trait to handle events from the Launchpad Pro.
impl LaunchpadApp for App {
    fn init_event(&self, _adc: hal::surface::Pads) {}
    fn timer_event(&self) {}
    fn midi_event(&self, _port: hal::midi::Port, _message: MidiMessage) {}
    fn sysex_event(&self, _port: hal::midi::Port, _data: &[u8]) {}
    fn cable_event(&self, _cable_event: hal::midi::CableEvent) {}
    fn button_event(&self, _button_event: hal::surface::ButtonEvent) {}
    fn aftertouch_event(&self, _aftertouch_event: hal::surface::AftertouchEvent) {}
}

#[cfg(target_arch="arm")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_arch="arm"))]
fn main() {}
