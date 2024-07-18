use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// * `threads` - the number of threads in the pool.
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            size,
            workers,
            sender,
        }
    }

    pub fn execute<F>(self: &Self, cb: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(cb);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            job();
        });

        Worker { id, thread }
    }
}
