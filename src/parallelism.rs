use crate::errors::PConvertError;
use image::{ImageBuffer, Rgba};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{spawn, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    work_channel_sender: mpsc::Sender<WorkMessage>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PConvertError> {
        if size == 0 {
            return Err(PConvertError::ArgumentError(
                "Thread Pool size should be a positive number".to_string(),
            ));
        }

        let (work_channel_sender, work_channel_receiver) = mpsc::channel();

        let work_channel_receiver = Arc::new(Mutex::new(work_channel_receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&work_channel_receiver)));
        }

        Ok(ThreadPool {
            workers,
            work_channel_sender,
        })
    }

    pub fn execute<F>(&self, func: F) -> mpsc::Receiver<ResultMessage>
    where
        F: FnOnce() -> ResultMessage + Send + 'static,
    {
        let (result_channel_sender, result_channel_receiver) = mpsc::channel();
        let task = Box::new(func);
        self.work_channel_sender
            .send(WorkMessage::NewTask(task, result_channel_sender))
            .unwrap_or_default();

        result_channel_receiver
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.work_channel_sender
                .send(WorkMessage::Terminate)
                .unwrap_or_default();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or_default();
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
                WorkMessage::NewTask(task, result_channel_sender) => {
                    let result = task();
                    result_channel_sender.send(result).unwrap_or_default();
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

pub enum ResultMessage {
    ImageResult(Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PConvertError>),
}
