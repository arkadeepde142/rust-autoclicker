// #![feature(once_cell)]
use autoclicker::Autoclicker;
use rdev::{listen, Event, EventType, Key};
use std::mem;
use std::sync::{Arc, Mutex, Once};

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<Autoclicker>>,
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(Autoclicker::new(
                    std::time::Duration::from_millis(1),
                ))),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

fn main() {
    listen(callback);
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::KeyW) => {
            if !singleton().inner.lock().unwrap().running() {
                singleton().inner.lock().unwrap().start().unwrap()
            } else {
                singleton().inner.lock().unwrap().stop().unwrap()
            }
        }
        EventType::KeyPress(Key::KeyS) => {
            println!("Exiting");
            std::process::exit(0);
        }
        _ => {}
    }
}
