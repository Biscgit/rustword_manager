use std::{
    thread::{self, JoinHandle},
    time::Duration,
    sync::{Arc, Mutex, mpsc},
};
use std::sync::mpsc::Sender;


const PRECISION: isize = 5;

pub enum Message {
    Stop,
    Reset,
}

pub struct Timer {
    default: isize,
    current: isize,
}

impl Timer {
    pub fn new(timeout: isize) -> Timer {
        Timer {
            default: timeout * PRECISION,
            current: timeout * PRECISION,
        }
    }

    pub fn reset_timer(&mut self) {
        self.current = self.default
    }

    pub fn stop_timer(&mut self) {
        self.current = -1;
    }

    pub fn decrease(&mut self) {
        self.default -= 1;
    }
}

pub struct TimerManager {
    sender: Option<Sender<Message>>,
    handle: Option<JoinHandle<()>>,
    shared_timer: Arc<Mutex<Timer>>,
}

impl TimerManager {
    pub fn new(timeout: isize) -> TimerManager {
        TimerManager {
            sender: None,
            handle: None,
            shared_timer: Arc::new(Mutex::new(Timer::new(timeout))),
        }
    }

    fn spawn_thread(&mut self) {
        let (sender, receiver) = mpsc::channel();
        let shared_timer = Arc::clone(&self.shared_timer);

        let handle = thread::spawn(move || {
            loop {
                match receiver.recv_timeout(Duration::from_millis(1000 / PRECISION as u64)) {
                    Ok(Message::Reset) => {
                        let mut timeout = shared_timer.lock().expect("Failed to acquire lock");
                        timeout.reset_timer();
                    }
                    Ok(Message::Stop) => {
                        let mut timeout = shared_timer.lock().expect("Failed to acquire lock");
                        timeout.stop_timer();

                        return;
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        let mut timeout = shared_timer.lock().expect("Failed to acquire lock");
                        timeout.decrease();
                    }

                    Err(_) => { panic!("Error receiving message. Exiting process."); }
                }
            }
        });

        self.sender = Some(sender);
        self.handle = Some(handle);
    }

    pub fn start_timer(&mut self) {
        if self.handle.is_some() && self.sender.is_some() {
            self.sender.as_ref().unwrap().send(Message::Reset).unwrap();
            self.sender.take();
        } else {
            self.spawn_thread();
        }
    }

    pub fn stop_timer(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.sender.take().unwrap().send(Message::Stop).expect("Failed to communicate with Timer");

            handle.join().unwrap();
        }
    }

    pub fn reset_timer(&mut self) {
        if let Some(sender) = &self.sender {
            sender.send(Message::Reset).unwrap();
        }
    }
}
