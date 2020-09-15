use std::sync::mpsc;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    thread: Option<JoinHandle<()>>,
}

type Task = Box<dyn FnOnce()>;
enum Message {
    NewTask(Task),
    Terminate,
}
