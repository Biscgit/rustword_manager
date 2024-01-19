use arboard::Clipboard;
use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{app::ClState, key_processor::SecureStorage};

// const that holds timeout of clipboard
const TIMEOUT: u64 = 30;

pub enum Message {
    // communication message sent to thread
    Stop,
    Reset(String),
}

pub struct ClipboardManager {
    // holds pipe, thread handle and shared clipboard
    sender: Option<mpsc::Sender<Message>>,
    handle: Option<JoinHandle<io::Result<()>>>,
    shared_clipboard: Arc<Mutex<Clipboard>>,
    shared_cl_state: ClState,
}

impl ClipboardManager {
    pub fn new(cl_state: ClState) -> ClipboardManager {
        // creates new empty manager
        ClipboardManager {
            sender: None,
            handle: None,
            shared_clipboard: Arc::new(Mutex::new(Clipboard::new().unwrap())),
            shared_cl_state: cl_state,
        }
    }

    pub fn copy_to_clipboard(&mut self, text: &str) {
        // function to call for copying a text to clipboard
        log::info!("Copied value to clipboard");
        self.reset_timer(text);

        let mut clipboard = self.shared_clipboard.lock().unwrap();
        clipboard.set_text(text).unwrap();
    }

    pub fn force_clear_clipboard(&mut self) {
        // function to call for resetting the clipboard
        self.stop_timer();
    }

    fn spawn_thread(&mut self, content: &str) {
        // spawns a new timer thread
        let (sender, receiver) = mpsc::channel();

        // init shared references
        let shared_clipboard = Arc::clone(&self.shared_clipboard);
        let shared_cl_state = Arc::clone(&self.shared_cl_state);

        // store last copied password in memory safely
        let mut current_pw = SecureStorage::from_string(content.to_string());

        // spawn new thread "Clipboard Clearer"
        let handle: JoinHandle<io::Result<()>> = thread::Builder::new()
            .name("Clipboard Clearer".to_string())
            .spawn(move || {
                // waits until timeout, restart if reset send through pipe, return if pipe dropped
                loop {
                    match receiver.recv_timeout(Duration::from_secs(TIMEOUT)) {
                        Ok(Message::Reset(new_pw)) => {
                            current_pw = SecureStorage::from_string(new_pw);
                        }
                        Ok(Message::Stop) => {
                            break;
                        }

                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            break;
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            return Ok(());
                        }
                    }
                }

                // clear clipboard if current password is still same (no new copies)
                let mut clipboard = shared_clipboard.lock().unwrap();
                let copied = String::from_utf8(current_pw.get_contents()).unwrap_or_default();

                if copied == clipboard.get_text().unwrap_or_default() {
                    log::info!("Clearing clipboard");

                    if clipboard.clear().is_err() {
                        log::warn!("Failed to clear clipboard");
                    }
                }

                // clear visual copied
                let mut clip_state = shared_cl_state.lock().unwrap();
                clip_state.value = None;

                Ok(())
            })
            .expect("Failed to spawn timer");

        // set values in ClipboardManager
        self.sender = Some(sender);
        self.handle = Some(handle);
    }

    fn reset_timer(&mut self, copy: &str) {
        // resets thread through pipe if it exists otherwise spawn a new with content
        if self.handle.is_some() && self.sender.is_some() {
            // possible error when thread is waiting for clipboard mutex. Stop and create new thread
            if self
                .sender
                .as_ref()
                .unwrap()
                .send(Message::Reset(copy.to_string()))
                .is_err()
            {
                self.handle.take().unwrap().join().unwrap().unwrap();
                self.spawn_thread(copy);
            }
        } else {
            self.spawn_thread(copy);
        }
    }

    fn stop_timer(&mut self) {
        // tries to stop thread through the message pipe if thread is running
        if let Some(handle) = self.handle.take() {
            self.sender
                .take()
                .unwrap()
                .send(Message::Stop)
                .unwrap_or(());

            if handle.join().is_err() {
                log::warn!("Failed to join clipboard thread");
            }
        }
    }
}

// pub struct Timer {
//     // holds current time of thread
//     default: isize,
//     current: isize,
// }
//
// impl Timer {
//     pub fn new(timeout: isize) -> Timer {
//         Timer {
//             default: timeout * PRECISION,
//             current: timeout * PRECISION,
//         }
//     }
//
//     pub fn reset_timer(&mut self) {
//         self.current = self.default
//     }
//
//     pub fn decrease(&mut self) -> isize {
//         if self.default >= 0 {
//             self.default -= 1;
//         }
//
//         self.default
//     }
// }
