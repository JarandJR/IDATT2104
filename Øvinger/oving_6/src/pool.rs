use std::collections::VecDeque;
use std::thread::JoinHandle;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

pub struct ThreadPool<F>
where
    F: FnOnce() + Send + 'static,
{
    number_of_workers: u32,
    threads: Vec<JoinHandle<()>>,
    tasks: Arc<Mutex<VecDeque<F>>>,
    running: Arc<Mutex<bool>>,
    condition_variable: Arc<(Mutex<bool>, Condvar)>,
}

impl<F> ThreadPool<F>
where
    F: FnOnce() + Send + 'static,
{
    pub fn new(number_of_workers: u32) -> Self {
        let mut pool = Self {
            number_of_workers,
            threads: Vec::new(),
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(Mutex::new(false)),
            condition_variable: Arc::new((Mutex::new(true), Condvar::new())),
        };
        pool.start_loop();
        pool
    }

    pub fn start_loop(&mut self) {
        //Checks if the loop is already running
        if *self.running.lock().unwrap() {
            return;
        }

        {
            //starts the loop
            *self.running.lock().unwrap() = true;
        }

        for _ in 0..self.number_of_workers {
            // Copies values so that thread does not take ownership of the variables
            let tasks_copy = self.tasks.clone();
            let running_copy = self.running.clone();
            let condition_variable_copy = self.condition_variable.clone();

            // Spawns the thread
            self.threads.push(thread::spawn(move || {
                println!("{:?} is alive", thread::current().id());

                while *running_copy.lock().unwrap() || !tasks_copy.lock().unwrap().is_empty() {
                    let (lock, c) = &*condition_variable_copy;
                    // Waits from signal by the condition variable
                    // The condition variable is true by default
                    // Therefor the thread is waiting until the conditional variable is set to false
                    {
                        let mut wait = lock.lock().unwrap();
                        while *wait {
                            wait = c.wait(wait).unwrap();
                            if !*running_copy.lock().unwrap() {
                                break;
                            }
                        }
                    }

                    // Ready to fetch and run tasks
                    while !tasks_copy.lock().unwrap().is_empty() {
                        // Defines task
                        let mut task: Option<F> = None;
                        {
                            // Sets task to a value of type fn()
                            let mut tasks = tasks_copy.lock().unwrap();
                            if !tasks.is_empty() {
                                task = tasks.pop_back();
                            }
                        }

                        // Checks if task is assigned to a vlaue of type fn()
                        // If true, then run the function
                        if let Some(task_to_run) = task {
                            task_to_run();
                        }
                    }
                    // Set the conditional variable back to default
                    *lock.lock().unwrap() = true;
                }
            }));
        }
    }

    pub fn post_task(&self, task: F) {
        self.tasks.lock().unwrap().push_front(task);
        self.notify_all_threads();
    }

    fn notify_all_threads(&self) {
        let (lock, c) = &*self.condition_variable;
        *lock.lock().unwrap() = false;
        c.notify_all();
    }
}
