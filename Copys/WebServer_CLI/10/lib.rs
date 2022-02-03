use std::{ sync::{mpsc, Arc, Mutex}, thread };

type Job= Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread= thread::spawn(move || loop {
            let message= receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} go a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} go a job; terminate.", id);
                    break;
                }
            }
        });
    }
}



pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let (sender, receiver)= mpsc::channel();

        let mut workers= Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver)
            ));
        }
        ThreadPool{ workers }
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

        println!("{}Shutting down all worker.");

        for worker in &mut self.Worker{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread::JoinHandle<()>)= worker.thread.take(){
                thread.join().unwrap();
            }

            worker.thread.join().unwrap();
        }
    }
}