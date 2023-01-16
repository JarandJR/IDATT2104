use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Find primes between: ");
    let from_input:u32 = get_number_from_user("from: ");
    let to_input:u32 = get_number_from_user("to: ");

    //If from is greater than to, they swap values
    let from = if to_input < from_input {to_input} else {from_input};
    let mut to = if to_input < from_input {from_input} else {to_input};

    if to % 2 != 0 {to += 1;}

    //Number of threads, max should be 2 because my computer only have 2 cores
    //But max is set to 32 for testing purposes
    let number_of_threads:u32 = get_valid_number();

    let primes = Arc::new(Mutex::new(Vec::new()));

    //Vector of threads
    let mut threads = Vec::new();
    for i in 0..number_of_threads {
        //Cloning variables because of closure after moving
        let primes_copy = primes.clone();
        let step = number_of_threads;
        let start = from + i;

        threads.push(thread::spawn(move || {
            let mut i = start;
            while i <= to {
                if is_prime(i) {primes_copy.lock().unwrap().push(i);}
                i += step;
            }
            })
        );
    }

    //Joining all the threads
    for t in threads {
        match t.join() {
            Ok(_) => continue,
            Err(_) => println!("Thread could not join.."),
        };
    }

    // Cannot access data without calling lock(), even though locking is unnecessary.
    let list = &mut *primes.lock().unwrap();
    list.sort();
    //Printing the result list
    for i in list { print!("{}, ", i);}
}

fn is_prime(n:u32) -> bool {
    for i in 2..n/2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn get_number_from_user(message:&str) -> u32 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Could not receive input");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn get_valid_number() -> u32{
    const MAX_NUMBER_OF_THREADS:u32 = 32;
    loop {
        println!("Number of threads (Max 32) ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Could not receive input");
        match input.trim().parse() {
            Ok(num) => {
                if num > MAX_NUMBER_OF_THREADS {continue;}
                return num },
            Err(_) => continue,
        };
    }
}