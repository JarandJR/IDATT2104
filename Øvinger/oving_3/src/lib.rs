use std::{thread, sync::{mpsc, Mutex, Arc}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    //Panics if size is less than zero
    pub fn new(size:usize) -> Self{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers =Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let task = Box::new(f);
        self.sender.send(Message::NewTask(task)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} is executing a task", id);
                
                match message {
                    Message::NewTask(task) => {
                        println!("Worker {} executing task", id);
                        task();
                    }
                    Message::Terminate => {
                        println!("Worker {} is terminating", id);
                        break;
                    }        
                }
            }))
        }
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewTask(Task),
    Terminate
}