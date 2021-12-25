use std::{ sync::mpsc, thread };
pub struct ThreadPool;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender : mpsc::Sender<Message>,
}

type Job= Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let (sender. receiver)= mpsc::channel();
        let mut Workers= Vec::with_capacity(size);
        for id in 0..size{
            Workers.push(Workers::new(
                id,
                Arc::clone(&receiver)
            ));
        }
        ThreadPool { Workers }
    }
    
    pub fn execute<F> (&self, f: F)
    where
        F
        {
            let job= Box::new(f);
            slef.sender.send(Message::NewJob(job)).unwrap();
        }
}

struct Worker{
    id: usize,
    threa: Option<thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::receiver<Job>>>) -> Worker {
        let thread= thread::spawn(move || loop {
            let Message= receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            match message{
                Message::NewJob(job: Box<dyn FnOnce() + Send>) => {
                    println!("Worker {} og a job; executing.", id);
                    job();
                }
                Message::Terninate => {
                    println!("Worker {} og a job; executing.", id);
                    break;
                }
            }
        });

        Worker{ id, thread: some(thread) }
    }
}