use std::sync::mpsc;
use std::thread;
use std::io;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new(tick: u32) -> Events {
        let (tx, rx) = mpsc::channel();

        let tx2 = tx.clone();

        thread::spawn(move || {
            let stdin = io::stdin();

            for key in stdin.keys().flatten() {
                if let Err(e) = tx.send(Event::Input(key)) {
                    eprintln!("Error while processing input events: {}", e);
                    return;
                }
            }
        });

        thread::spawn(move || loop {
            if let Err(e) = tx2.send(Event::Tick) {
                eprintln!("Error while processing tick: {}", e);
                return
            }

            thread::sleep(Duration::from_millis(u64::from(tick)));
        });
        
        Events { rx }
    }

    pub fn get_event(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
