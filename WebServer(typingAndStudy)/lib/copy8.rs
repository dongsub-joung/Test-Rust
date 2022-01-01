pub struct ThreadPool;

use std::{ sync::mpsc, thread }
// use std::sync::mpsc;
// use std::sync::thread;

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job= Box<dyn FnOnce() + Send + 'static>

enum Message {
    NewJob(job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0 );
        let (sender, receiver)= mpsc::channel();
        let mut workers= Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver)
            ));
        }
        ThreadPool { workers }
    }

    pub fn execute<F> (&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job= Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
    }
}