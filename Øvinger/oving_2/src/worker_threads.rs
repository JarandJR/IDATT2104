pub struct WorkerThreads {
    threads: Vec<u32>
}

impl WorkerThreads {
    pub fn new() -> WorkerThreads {
        Self { threads: (Vec::new()) }
    }

    pub fn add(&mut self, value: u32) {
        self.threads.push(value);
        self.print_added();
    }

    pub fn get(&self) -> &Vec<u32>{
        &self.threads
    }

    fn print_added(&self) {
        println!("added value")
    }
}