mod worker_threads;

use crate::worker_threads::WorkerThreads;

fn main() {
    println!("Finally works");
    let rectangle = Square{
        width: 2.0,
        height: 4.0,
    };
    println!("width {}, height {}", rectangle.width, rectangle.height);

    let square = Square::new(2.0, 2.0);
    square.print_self();
    println!("{}", square.get_area());

    let mut workers = WorkerThreads::new();
    workers.add(2);
    workers.add(53);
    for v in workers.get() {
        println!("v: {}", v);
    }
}

//Define parameters
struct Square {
    width: f32,
    height: f32,
}

//Interface, kinda
trait Formula {
    fn get_area(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_width(&self) -> f32;
}

//Implement interface
impl Formula for Square {
    fn get_area(&self) -> f32 {
        self.width * self.height
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_width(&self) -> f32 {
        self.width
    }
}

//implement methods in struct
impl Square {
    //constructor kinda
    fn new(width: f32, height: f32) -> Square {
        Square { width: (width), height: (height) }
    }

    //scuffed to string
    fn print_self(&self) {
        println!("width: {}, height: {}", self.width, self.height);
    }
}
