use crate::constants;
use crate::errors::PConvertError;
use crate::utils::min;
use image::{ImageBuffer, Rgba};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{spawn, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    work_channel_sender: mpsc::Sender<WorkMessage>,
    work_channel_receiver_mutex: Arc<Mutex<mpsc::Receiver<WorkMessage>>>,
    status: Arc<ThreadPoolStatus>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PConvertError> {
        if size == 0 {
            return Err(PConvertError::ArgumentError(
                "Thread Pool size should be a positive number".to_string(),
            ));
        }
        let (work_channel_sender, work_channel_receiver) = mpsc::channel();
        let workers = Vec::with_capacity(size);
        let work_channel_receiver_mutex = Arc::new(Mutex::new(work_channel_receiver));

        let status = Arc::new(ThreadPoolStatus::new(size));

        Ok(ThreadPool {
            workers,
            work_channel_sender,
            work_channel_receiver_mutex,
            status,
        })
    }

    pub fn start(&mut self) {
        for _ in 0..self.workers.capacity() {
            self.spawn_worker();
        }
    }

    fn stop(&mut self) {
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

    pub fn execute<F>(&self, func: F) -> mpsc::Receiver<ResultMessage>
    where
        F: FnOnce() -> ResultMessage + Send + 'static,
    {
        let (result_channel_sender, result_channel_receiver) = mpsc::channel();
        let task = Box::new(func);
        self.work_channel_sender
            .send(WorkMessage::NewTask(task, result_channel_sender))
            .unwrap_or_default();

        self.status.inc_queued_count();

        result_channel_receiver
    }

    pub fn expand_to(&mut self, num_threads: usize) {
        let num_threads = min(
            num_threads as isize,
            constants::MAX_THREAD_POOL_SIZE as isize,
        );
        let to_spawn = num_threads - self.status.size() as isize;
        for _ in 0..to_spawn {
            self.spawn_worker();
            self.status.inc_size();
        }
    }

    pub fn get_status(&self) -> ThreadPoolStatus {
        (*self.status).clone()
    }

    fn spawn_worker(&mut self) {
        self.workers.push(Worker::new(
            self.status.clone(),
            Arc::clone(&self.work_channel_receiver_mutex),
        ));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}

struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(
        thread_pool_status: Arc<ThreadPoolStatus>,
        receiver: Arc<Mutex<mpsc::Receiver<WorkMessage>>>,
    ) -> Worker {
        let thread = spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                WorkMessage::NewTask(task, result_channel_sender) => {
                    thread_pool_status.dec_queued_count();
                    thread_pool_status.inc_active_count();

                    let result = task();
                    std::thread::sleep_ms(2000); //Temporary to demonstrate py.allow_threads effect
                    result_channel_sender.send(result).unwrap_or_default();

                    thread_pool_status.dec_active_count();
                }

                WorkMessage::Terminate => {
                    thread_pool_status.dec_size();
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

pub struct ThreadPoolStatus {
    size: AtomicUsize,
    queued_count: AtomicUsize,
    active_count: AtomicUsize,
}

impl ThreadPoolStatus {
    pub fn new(size: usize) -> Self {
        ThreadPoolStatus {
            size: AtomicUsize::new(size),
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
        }
    }

    pub fn size(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }

    pub fn queued(&self) -> usize {
        self.queued_count.load(Ordering::Relaxed)
    }

    pub fn active(&self) -> usize {
        self.active_count.load(Ordering::Relaxed)
    }

    pub fn inc_queued_count(&self) {
        self.queued_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec_queued_count(&self) {
        self.queued_count.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn inc_active_count(&self) {
        self.active_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec_active_count(&self) {
        self.active_count.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn inc_size(&self) {
        self.size.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec_size(&self) {
        self.size.fetch_sub(1, Ordering::Relaxed);
    }
}

impl Clone for ThreadPoolStatus {
    fn clone(&self) -> Self {
        ThreadPoolStatus {
            size: AtomicUsize::new(self.size()),
            queued_count: AtomicUsize::new(self.queued()),
            active_count: AtomicUsize::new(self.active()),
        }
    }
}
