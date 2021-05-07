use enigo::*;
use std::cmp::PartialEq;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(std::fmt::Debug)]
pub struct Autoclicker {
    sender: Sender<Message>,
    worker: Option<std::thread::JoinHandle<()>>,
    receiver: Arc<Mutex<Receiver<Message>>>,
    delay: u64,
}

#[derive(PartialEq)]
pub enum Message {
    Stop,
}

impl Drop for Autoclicker {
    fn drop(&mut self) {
        if let Ok(()) = self.stop() {
            println!("Shutting down");
        }
    }
}

impl Autoclicker {
    pub fn new(delay: u64) -> Autoclicker {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        Autoclicker {
            sender: tx,
            receiver: rx,
            worker: None,
            delay: delay,
        }
    }

    pub fn start(&mut self) -> Result<(), ()> {
        let delay = self.delay;
        let rx = Arc::clone(&self.receiver);
        self.worker = Some(thread::spawn(move || {
            let mut enigo = Enigo::new();
            loop {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(std::time::Duration::from_millis(delay));
                if Ok(Message::Stop) == (*rx.lock().unwrap()).try_recv() {
                    return;
                }
            }
        }));
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), ()> {
        let _ = self.sender.send(Message::Stop);
        if let Some(thread) = self.worker.take() {
            thread.join().unwrap();
            self.worker = None;
            return Ok(());
        }
        Err(())
    }

    pub fn running(&self) -> bool {
        if let Some(_) = self.worker {
            return true;
        }
        false
    }
}

#[test]
fn auto_clicker_bot_test() {
    let mut autoclicker = Autoclicker::new(200);
    assert_eq!(autoclicker.running(), false);
    assert_eq!(autoclicker.start(), Ok(()));
    assert_eq!(autoclicker.running(), true);
    assert_eq!(autoclicker.delay, 200);
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert_eq!(autoclicker.stop(), Ok(()));
    assert_eq!(autoclicker.running(), false);
}
