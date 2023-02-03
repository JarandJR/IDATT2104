use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

pub struct WorkerThreads {
    number_of_workers: u32,
    threads: Vec<JoinHandle<()>>,
    tasks: Arc<Mutex<Vec<fn()>>>,
    running: Arc<Mutex<bool>>,
    condition_variable: Arc<(Mutex<bool>, Condvar)>,
}

impl WorkerThreads {
    pub fn new(number_of_workers: u32) -> WorkerThreads {
        WorkerThreads {
            number_of_workers,
            threads: Vec::new(),
            tasks: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(Mutex::new(false)),
            condition_variable: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn start_loop(&mut self) {
        //Checks if the loop is already running
        if *self.running.lock().unwrap() {
            return;
        }

        {
            *self.running.lock().unwrap() = true;
        }

        for _ in 0..self.number_of_workers {
            let tasks = self.tasks.clone();
            let running = self.running.clone();
            let condition_variable = self.condition_variable.clone();

            self.threads.push(thread::spawn(move || {
                println!("I am alive {:?}", thread::current().id());
                println!("I am dying {:?}", thread::current().id());
            }));
        }
    }

    pub fn add_task(&self, task: fn()) {
        self.tasks.lock().unwrap().push(task);
        self.print_added();
        self.notify_all_threads();
    }

    fn print_added(&self) {
        println!("task added")
    }

    fn notify_one_thread(&self) {}

    fn notify_all_threads(&self) {
        println!("notifyed all");
    }

    pub fn post_timeout(&self) {}

    pub fn end_loop(&self) {
        *self.running.lock().unwrap() = false;
    }
}
