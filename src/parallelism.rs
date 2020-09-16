use crate::errors::PConvertError;
use image::{ImageBuffer, Rgba};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{spawn, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkMessage>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PConvertError> {
        if size == 0 {
            return Err(PConvertError::ArgumentError(
                "Thread Pool size should be a positive number".to_string(),
            ));
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, func: F, return_channel: mpsc::Sender<ResultMessage>)
    where
        F: FnOnce() -> ResultMessage + Send + 'static,
    {
        let task = Box::new(func);
        self.sender
            .send(WorkMessage::NewTask(task, return_channel))
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending <Terminate> messages to all workers");
        for _ in &self.workers {
            self.sender.send(WorkMessage::Terminate).unwrap();
        }

        println!("Shutting down all workers");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<WorkMessage>>>) -> Worker {
        let thread = spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                WorkMessage::NewTask(task, return_channel) => {
                    let result = task();
                    return_channel.send(result).unwrap();
                }

                WorkMessage::Terminate => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

type Task = Box<dyn FnOnce() -> ResultMessage + Send>;
enum WorkMessage {
    NewTask(Task, mpsc::Sender<ResultMessage>),
    Terminate,
}

#[derive(Debug)]
pub enum ResultMessage {
    ImageResult(Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PConvertError>),
}
