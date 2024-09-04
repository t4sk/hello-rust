use std::io::Write;
use std::{thread, time};
// mpsc - multi producer single consumer
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

pub struct Spinner {
    tx: Sender<Option<String>>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.tx.send(None).unwrap();
        self.join_handle.take().unwrap().join().unwrap();
    }
}

impl Spinner {
    pub fn start() -> Self {
        let frames = ["-", "\\", "|", "/"];

        // transmitter and receiver
        let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = channel();
        let mut stream = std::io::stdout();

        let join_handle = thread::spawn(move || {
            loop {
                for f in frames.iter() {
                    // Does not block
                    let (stop, msg) = match rx.try_recv() {
                        Ok(msg) => (true, msg),
                        Err(TryRecvError::Disconnected) => (true, None),
                        Err(TryRecvError::Empty) => (false, None),
                    };

                    if stop {
                        if let Some(msg) = msg {
                            println!("\r{}", msg);
                        }
                        return;
                    }

                    write!(stream, "\r{}", f).unwrap();
                    stream.flush().unwrap();

                    thread::sleep(time::Duration::from_millis(200));
                }
            }
        });

        Self {
            tx,
            join_handle: Some(join_handle),
        }
    }

    pub fn stop(&mut self) {
        self.tx.send(Some(String::from("stopped"))).unwrap();
    }
}

fn main() {
    let mut spinner = Spinner::start();
    thread::sleep(time::Duration::from_secs(3));
    // Spinner is dropped here so no need to call stop
    spinner.stop();
}
