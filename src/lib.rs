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
    delay: Arc<Mutex<std::time::Duration>>,
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
    pub fn new(delay: std::time::Duration) -> Autoclicker {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        Autoclicker {
            sender: tx,
            receiver: rx,
            worker: None,
            delay: Arc::new(Mutex::new(delay)),
        }
    }

    pub fn start(&mut self) -> Result<(), ()> {
        let delay = Arc::clone(&self.delay);
        let rx = Arc::clone(&self.receiver);
        self.worker = Some(thread::spawn(move || {
            let mut enigo = Enigo::new();
            loop {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(*(delay.lock().unwrap()));
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
