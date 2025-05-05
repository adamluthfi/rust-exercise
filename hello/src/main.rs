
use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::thread;

use hello::method1;
use hello::do_stuff;
use rand::random_range;

fn main() {
    method1();
    let x = random_range(0.0..=1e9);
    println!("{}", x);

    let mut s1 = String::from("abc");
    do_stuff(&mut s1);

    println!("{}", s1);

    let fox = RedFox {
        enemy: true,
        life: 70,
    };
    println!("{}", fox.enemy);
    println!("{}", fox.life);

    let fox = RedFox::new();
    let life_left = fox.life;
    println!("{}", life_left);

    print_noise(5_u8);

    let robot = Robot {};
    robot.run();

    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);

    let mut x = v.pop();
    if let Some(value) = x.take() {
        println!("{}", value);
    }
    
    println!("{}", v[1]);

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(6, true);
    h.insert(7, false);
    if let Some(value) = h.get(&6) {
        println!("Got {}", value)
    }

    let handle = thread::spawn(move || {

    });
    handle.join().unwrap();
}

struct  RedFox {
    enemy: bool,
    life: u8
}

impl RedFox {
    fn new() -> RedFox {
        RedFox { enemy: true, life: 70 }
    }
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

trait Run {
    fn run(&self) {
        println!("I'm Running")
    }
}

struct Robot {}
impl Run for Robot {}