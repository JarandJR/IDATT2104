mod worker_threads;

use crate::worker_threads::WorkerThreads;
use std::thread;

fn main() {
    let mut workers = WorkerThreads::new(4);

    println!("STARTING LOOP");
    workers.start_loop();

    println!("SLEEPS FOR 4 s");
    thread::sleep(std::time::Duration::from_secs(4));

    println!("POSTING TASKS");
    for _i in 0..5 {
        workers.post_task(task);
    }
    
    println!("POSTING TASKS WITH TIMEOUT");
    workers.post_timeout(task, 3);
    workers.post_timeout(task,3);

    println!("ENDING LOOP");
    workers.end_loop();
}

fn task() {
    println!("Task is worked on by {:?}", thread::current().id());
    thread::sleep(std::time::Duration::from_secs(3));
}
