mod singletonbuilder;

use autoclicker::Autoclicker;
use rdev::{listen, Event, EventType, Key};

fn main() {
    let global = singleton();
    let mut autoclicker = global.autoclicker.lock().unwrap();
    *autoclicker = Autoclicker::new(200);
    listen(callback);
}

fn _ui_builder() {}

use singletonbuilder::singleton;
fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::KeyW) => {
            let global = singleton();
            let mut autoclicker = global.autoclicker.lock().unwrap();

            if !autoclicker.running() {
                autoclicker.start().unwrap()
            } else {
                autoclicker.stop().unwrap()
            }
        }
        EventType::KeyPress(Key::KeyS) => {
            println!("Exiting");
            std::process::exit(0);
        }
        _ => {}
    }
}
