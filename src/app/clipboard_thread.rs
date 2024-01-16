use arboard::Clipboard;
use std::{
    thread::{self, JoinHandle},
    time::Duration,
    sync::{Arc, Mutex, mpsc},
    io,
};

use crate::{app::ClState, key_processor::SecureStorage};


const PRECISION: isize = 5;
const TIMEOUT: isize = 5;

pub enum Message {
    Stop,
    Reset(String),
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

    pub fn decrease(&mut self) -> isize {
        if self.default >= 0 {
            self.default -= 1;
        }

        self.default
    }
}

pub struct ClipboardManager {
    sender: Option<mpsc::Sender<Message>>,
    handle: Option<JoinHandle<io::Result<()>>>,
    shared_clipboard: Arc<Mutex<Clipboard>>,
    shared_cl_state: ClState,
}

impl ClipboardManager {
    pub fn new(cl_state: ClState) -> ClipboardManager {
        ClipboardManager {
            sender: None,
            handle: None,
            shared_clipboard: Arc::new(Mutex::new(Clipboard::new().unwrap())),
            shared_cl_state: cl_state,
        }
    }

    pub fn copy_to_clipboard(&mut self, text: &str) {
        self.reset_timer(text);

        let mut clipboard = self.shared_clipboard.lock().unwrap();
        clipboard.set_text(text).unwrap();
    }

    pub fn force_clear_clipboard(&mut self) {
        self.stop_timer();
    }

    fn spawn_thread(&mut self, content: &str) {
        let (sender, receiver) = mpsc::channel();
        let shared_clipboard = Arc::clone(&self.shared_clipboard);
        let shared_cl_satet = Arc::clone(&self.shared_cl_state);

        // create timer to be sent to thread
        let mut timer = Timer::new(TIMEOUT);
        let mut current_pw = SecureStorage::from_string(content.to_string());

        let handle: JoinHandle<io::Result<()>> = thread::Builder::new()
            .name("Clipboard clearer".to_string())
            .spawn(move || {
                loop {
                    match receiver.recv_timeout(Duration::from_millis(1000 / PRECISION as u64)) {
                        Ok(Message::Reset(new_pw)) => {
                            timer.reset_timer();
                            current_pw = SecureStorage::from_string(new_pw);
                        }
                        Ok(Message::Stop) => { break; }

                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => { return Ok(()); }
                    }

                    if timer.decrease().is_negative() { break; }
                }

                // only clear if same password
                let mut clipboard = shared_clipboard.lock().unwrap();
                let copied = String::from_utf8(current_pw.get_contents()).unwrap_or(String::new());

                if copied == clipboard.get_text().unwrap_or(String::new()) {
                    if let Err(_err) = clipboard.clear() {
                        // ToDo: log if failed to clear clipboard
                    }
                }

                // clear visual copied
                let mut cl_state = shared_cl_satet.lock().unwrap();
                cl_state.value = None;

                Ok(())
            })
            .expect("Failed to spawn timer");

        self.sender = Some(sender);
        self.handle = Some(handle);
    }

    fn reset_timer(&mut self, copy: &str) {
        if self.handle.is_some() && self.sender.is_some() {
            if let Err(_) = self.sender.as_ref().unwrap().send(Message::Reset(copy.to_string())) {
                self.handle.take().unwrap().join().unwrap().unwrap();
                self.spawn_thread(copy);
            }
        } else {
            self.spawn_thread(copy);
        }
    }

    fn stop_timer(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.sender.take().unwrap().send(Message::Stop).unwrap_or(());

            if let Err(_err) = handle.join() {
                // ToDo: log if failed to join thread
            }
        }
    }
}
