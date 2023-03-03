use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
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

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum HTTPTag {
    HOST,
    CONNECTION,
    CacheControl,
    UPGRADE,
    UserAgent,
    AcceptEncoding,
    AcceptLanguage,
    SecWebSocketKey,

    UNDEFINED,
}

impl HTTPTag {
    pub fn from_string(s: &str) -> Result<HTTPTag, &str> {
        match s {
            "Host:" => Ok(HTTPTag::HOST),
            "Connection:" => Ok(HTTPTag::CONNECTION),
            "Cache-Control:" => Ok(HTTPTag::CacheControl),
            "Upgrade-Insecure-Requests:" => Ok(HTTPTag::UPGRADE),
            "Upgrade:" => Ok(HTTPTag::UPGRADE),
            "User-Agent:" => Ok(HTTPTag::UserAgent),
            "Accept-Encoding:" => Ok(HTTPTag::AcceptEncoding),
            "Accept-Language:" => Ok(HTTPTag::AcceptLanguage),
            "Sec-WebSocket-Key:" => Ok(HTTPTag::SecWebSocketKey),
            _ => Ok(HTTPTag::UNDEFINED),
        }
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl Method {
    pub fn from_string(s: &str) -> Result<Method, &str> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err("Failed to parse method from string"),
        }
    }
}

pub struct HTTPRequest {
    pub method: Method,
    pub headers: HashMap<HTTPTag, String>,
}

impl HTTPRequest {
    pub fn new(request: &str) -> Self {
        let lines: Vec<&str> = request.split('\n').collect();
        let method = Method::from_string(
            lines
                .first()
                .expect("Could not get first line")
                .split(" ")
                .nth(0)
                .expect("Could not get element"),
        )
        .expect("Could not parse method");

        let mut headers: HashMap<HTTPTag, String> = HashMap::new();
        for l in &lines {
            let mut s = l.split(" ");
            let key = s.nth(0);

            let mut temp = String::new();
            let mut words = s.clone();
            let value = if s.count() > 1 {
                for w in words {
                    if temp.len() > 0 {
                        temp = format!("{temp} {w}");
                    } else {
                        temp = w.to_string();
                    }
                }
                Some(temp.as_str())
            } else {
                words.nth(0)
            };

            if key.is_some() && value.is_some() {
                let key = HTTPTag::from_string(key.expect("Could not split value"))
                    .expect("Could not parse into tag");
                let value = String::from(value.expect("Could not split value correctly"));
                headers.insert(key, value);
            }
        }
        //Just for debug
        for l in &lines {
            if !l.starts_with("\0\0") {
                println!("{}", l);
            }
        }
        Self { method, headers }
    }

    pub fn get_header_value_string(&mut self, key: &str) -> String {
        self.headers
            .remove(&HTTPTag::from_string(key).expect("Could not parse into tag"))
            .expect("Could not find key value")
    }

    pub fn get_header_value_key(&mut self, key: HTTPTag) -> String {
        self.headers.remove(&key).expect("Could not find key value")
    }
}

pub struct SocketRequest {
    http_request: HTTPRequest,
    sec_key: String,
    GUID: String,
}

impl SocketRequest {
    pub fn new(mut http_request: HTTPRequest) -> Self {
        let header_value = http_request.get_header_value_key(HTTPTag::SecWebSocketKey);
        Self {
            http_request,
            sec_key: header_value,
            GUID: String::from("insert key here"),
        }
    }
}
