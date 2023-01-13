#![allow(unused)]

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Find primes between: ");
    let from_input:u32 = get_number_from_user("from: ");
    let to_input:u32 = get_number_from_user("to: ");

    //If from is greater than to, they swap values
    let from = if ( to_input < from_input) {to_input} else {from_input};
    let to = if ( to_input < from_input) {from_input} else {to_input};

    let list_of_primes_mutex_arc = Arc::new(Mutex::new(Vec::new()));
    let index_at_mutex_arc = Arc::new(Mutex::new(from));    

    let list_of_primes_mutex_arc_copy = list_of_primes_mutex_arc.clone();
    let index_at_mutex_arc_copy = index_at_mutex_arc.clone();
    //Making thread 1
    let t1 = thread::spawn(move || {
        let mut index_at_locked = index_at_mutex_arc_copy.lock().unwrap();
        while (*index_at_locked < to + 1) {
        // Accessing and locking the data
        let mut list_of_primes_locked = list_of_primes_mutex_arc_copy.lock().unwrap();
        if (is_prime(*index_at_locked)) {list_of_primes_locked.push(*index_at_locked);} 
        *index_at_locked += 1;
        // Lock is released at end of scope
        }
    });

    let list_of_primes_mutex_arc_copy = list_of_primes_mutex_arc.clone();
    let index_at_mutex_arc_copy = index_at_mutex_arc.clone();
    //Making thread 2
    let t2 = thread::spawn(move || {
        let mut index_at_locked = index_at_mutex_arc_copy.lock().unwrap();
        while (*index_at_locked < to + 1) {
        // Accessing and locking the data
        let mut list_of_primes_locked = list_of_primes_mutex_arc_copy.lock().unwrap();
        if (is_prime(*index_at_locked)) {list_of_primes_locked.push(*index_at_locked);} 
        *index_at_locked += 1;
        // Lock is released at end of scope
        }
    });


    t1.join();
    t2.join();

    // Cannot access data without calling lock(), even though locking is unnecessary.
    let mut list = &mut *list_of_primes_mutex_arc.lock().unwrap();
    &list.sort();
    for i in list { print!("{}, ", i);}
}

fn is_prime(n:u32) -> bool {
    for i in (2..n) {
        if (n % i == 0) {
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
        let mut input:u32 = match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}