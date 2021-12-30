pub struct ThreadPool;

use std::{ sync::mpsc, thread };

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job= Box<dyn FnOnce() + Send + 'static>;

enum Message{
    NewJob(job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver)= mpsc::channel();
    } 
}