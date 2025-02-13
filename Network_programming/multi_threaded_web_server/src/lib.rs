use std::thread;
use std::sync::{
    mpsc, Arc, Mutex
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// struct Job;
type Job = Box<dyn FnOnce()+Send+'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender}
    }
    pub fn execute<F>(&self, f: F)
    where F: FnOnce()+Send+'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            println!("Worker {} got a job; executing.", id);
            job()
        });
        Worker {id, thread}
    }
}