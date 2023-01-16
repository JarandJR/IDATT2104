use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Find primes between: ");
    let from_input:u32 = get_number_from_user("from: ");
    let to_input:u32 = get_number_from_user("to: ");

    //If from is greater than to, they swap values
    let from = if to_input < from_input {to_input} else {from_input};
    let to = if to_input < from_input {from_input} else {to_input};

    //Getting the number of threads from user with a max of 32
    let number_of_threads:u32 = get_valid_number(from, to);

    let index_at_mutex_arc = Arc::new(Mutex::new(from));
    let list_of_primes_mutex_arc = Arc::new(Mutex::new(Vec::new()));

    //Vector of threads
    let mut vec = Vec::new();
    for i in 0..number_of_threads {
        //Cloning variables because of closure after moving
        let index_at_mutex_arc_copy = index_at_mutex_arc.clone();
        let list_of_primes_mutex_arc_copy = list_of_primes_mutex_arc.clone();
    
        println!("Making thread: {}", i);
        let t = thread::spawn(move || {
            let mut index_at_locked = index_at_mutex_arc_copy.lock().unwrap();
            println!("Locking at thread: {}", i);
            while *index_at_locked < to + 1 {
                // Accessing and locking the data
                let mut list_of_primes_locked = list_of_primes_mutex_arc_copy.lock().unwrap();
                if is_prime(*index_at_locked) {list_of_primes_locked.push(*index_at_locked);} 
                *index_at_locked += 1;
                // Lock is released at end of scope
            }
            println!("Finished thread: {}\n", i);
        });
       vec.push(t);
    }

    //Joining all the threads
    for v in vec {
        v.join();
    }

    // Cannot access data without calling lock(), even though locking is unnecessary.
    let list = &mut *list_of_primes_mutex_arc.lock().unwrap();
    //Do not need to sort the list because the list is already sorted
    //&list.sort();
    //Printing the result list
    for i in list { print!("{}, ", i);}
}

fn is_prime(n:u32) -> bool {
    for i in 2..n {
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

fn get_valid_number(from:u32, to:u32 ) -> u32 {
    let max_number_of_threads:u32 = to - from - 1;
    loop {
        let number_of_threads:u32 = get_number_from_user("Number of threads");
        if number_of_threads > max_number_of_threads {
            continue;
        }
        return number_of_threads;
    }
}