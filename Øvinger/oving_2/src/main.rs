mod worker_threads;

use crate::worker_threads::WorkerThreads;
use std::thread;

fn main() {
    let mut workers = WorkerThreads::new(4);
    workers.start_loop();
    thread::sleep(std::time::Duration::from_secs(4));
    for _i in 0..5 {
        workers.post_task(task);
    }
    thread::sleep(std::time::Duration::from_secs(4));
    workers.post_timeout(task, 2);
    workers.post_timeout(task, 1);
    println!("ENDING LOOP");
    workers.end_loop();
}

fn task() {
    println!("Task is worked on by {:?}", thread::current().id());
    thread::sleep(std::time::Duration::from_secs(3));
}
