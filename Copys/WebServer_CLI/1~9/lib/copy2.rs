pub struct ThreadPool;

use std::{ sync::mpsc, thread };

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job= Box<dyn FnOnce() + Send + 'static>;

enum Message{
    NewJob(Job),
    Terminate,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size> 0);

        let (sender, receiver)= mpsc::channel();

        let mut Workers= Vec::with_capacity(size);
        for id in 0..size{
            Worker.push(Workers::new(
                id,
                Arc::clone(&receiver)
            ));
        }
        ThreadPool{ Worker }
    }

    pub fn execute<F> (&self, f: F){
        where
            F: FnOnce() + Send + 'static
        {
            let job= Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap();
        }
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.")
        for _ in &self.worker{
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all worker.");
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread::JoinHandle<()>)= worker.thread.take(){
                thread.join().unwrap();
            }
            worker.thread.join().unwrap();
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mps::receiver<Job>>>) -> Worker{
        let thread= thread::spawn(move || loop {
            let Message= receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            
            match message{
                Message::NewJob(job: Box<dyn FnOnce()+Send) => {
                    println!("Worker {} go a job; executing.", id);
                    job();
                }
                Message:Terminate => {
                    println!("Worker {} go a job; terminate.", id);
                    break;
                }
            }
        });

        Worker{ id, thread: some(thread) }
    }
}