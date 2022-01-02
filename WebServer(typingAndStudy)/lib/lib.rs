// https://www.youtube.com/watch?v=1AamFJGAE8E&t  
// https://www.youtube.com/watch?v=qjx8vutWaUQ
// None adjuted Auto importing(syntax, type) system, manual tying

pub struct ThreadPool;

use std::{sync::mpsc, thread};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job= Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

// Testing for execute()
impl ThreadPool{
    // #Panices
    // The 'new' function will panic if the size if zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender: Sender<Job>, receiver: receiver<Job>)= 
            mpsc::channel();

        let mut workers= Vec<{unknow}>} = Vec::with_capacity(size);
        for id in 0..size {
            // create threads
            workers.push(Workers::new(
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
        let job: Box<F>= Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();~
        }

        println!("Shutting down all worker.");  

        for worker: &mut Worker in  &mut self.Worker {
            println!("Shutting down worker {}", worker.id);  
        
            if let Some(thread:: JoinHandle<()>)= worker.thread.take(){
                thread.join().unwrap();
            }
            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::receiver<Job>>>) -> Worker{
        let thread: JoinHandle<()>= thread::spawn( move || loop {
            // Recommand rewirte to auto type check system
            let message: Box.. =receiver: Arc<Mutex<receiver..>>
                .lock(): Result<MutexGuard<Receiver<...>>, >
                .unwrap() :MutexGuard<Receiver<Box<dyn FnOnce() + Send>>>
                .recv() : Result<Box<dyn FnOnce() + Send>, ..>
                .unwrap();

            match message{
                Message::NewJob(job: Box<dyn FnOnce() + Send>) => {
                    println!("Worker {} go a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} go a job; terminate.", id);
                    break;
                }
            }
        });

        Worker{id, thread: some(thread)}
    }
}