// https://www.youtube.com/watch?v=1AamFJGAE8E&t=519s
// None adjuted Auto importing(syntax, type) system, manual tying

pub struct ThreadPool;

use std::{sync::mpsc, thread};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job= Box<dyn FnOnce() + Send + 'static>;

// Testing for execute()
impl ThreadPool{
    // #Panices
    // The 'new' function will panic if the size if zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender: Sender<Job>, receiver: receiver<Job>)= 
            mpsc::channel();

        let mut Workers= Vec<{unknow}>} = Vec::with_capacity(size);
        for id in 0..size {
            // create threads
            Workers.push(Workers::new(
                id,
                Arc::clone(&receiver)
            ));
        }
        ThreadPool { Workers }
    }

    pub fn execute<F> (&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job: Box<F>= Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::receiver<Job>>>) -> Worker{
        let thread: JoinHandle<()>= thread::spawn( move || loop {
            // Recommand rewirte to auto type check system
            let job: Box.. =receiver: Arc<Mutex<receiver..>>
                .lock(): Result<MutexGuard<Receiver<...>>, >
                .unwrap() :MutexGuard<Receiver<Box<dyn FnOnce() + Send>>>
                .recv() : Result<Box<dyn FnOnce() + Send>, ..>
                .unwrap();

                println!("Worker {} go a job; executing.", id);
                job();
        });

        Worker{id, thread}
    }
}