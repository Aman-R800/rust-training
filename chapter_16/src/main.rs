use std::sync::mpsc::{Receiver, RecvError};
use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

struct Worker{
    id: u8,
    receiver: Arc<Mutex<Receiver<u8>>>,
}

impl Worker{
    fn run(&self){
        loop{
            let message = {
                let receiver = self.receiver.lock().unwrap();
                receiver.recv()
            };

            match message{
                Ok(num) => {
                    println!("Worker {} received {}", self.id, num);
                    thread::sleep(Duration::from_millis(100))
                },
                Err(RecvError) => {
                    println!("Sender dropped");
                    break
                }
            }
        }
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel::<u8>();
    let mut handle_vec:Vec<JoinHandle<()>> = vec![];

    let receiver = Arc::new(Mutex::new(receiver));


    for i in 0..5{
        let worker = Worker {id: i, receiver: receiver.clone()};
        let handle = thread::spawn(move || { worker.run() });

        handle_vec.push(handle)
    }
    

    for _ in 0..50{
        let gen = rand::random::<u8>();
        sender.send(gen).unwrap()
    }

    drop(sender);

    for handle in handle_vec{
        handle.join().unwrap()
    }
}
