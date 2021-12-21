// https://www.youtube.com/watch?v=1AamFJGAE8E&t=519s

pub struct ThreadPool;

use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

// Testing for execute()
impl ThreadPool{
    // #Panices
    // The 'new' function will panic if the size if zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads= Vec<{unknow}>} = Vec::with_capacity(size);
        for _ in 0..size {
            // create threads
        }
        ThreadPool { threads }
    }

    pub fn execute<F> (&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Worker{
        let thread: JoinHandle<()>= thread::spawn(|| {});

        Worker{id, thread}
    }
}