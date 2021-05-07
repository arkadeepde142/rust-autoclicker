use autoclicker::Autoclicker;
use std::mem;
use std::sync::{Arc, Mutex, Once};

use rdev::{listen, Event, EventType, Key};
#[derive(Clone)]
pub struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    pub autoclicker: Arc<Mutex<Autoclicker>>,
}
pub fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                autoclicker: Arc::new(Mutex::new(Autoclicker::new(1))),
            };
            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
            std::thread::spawn(move || {
                listen(callback);
            });
        });
        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

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
