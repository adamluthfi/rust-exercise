
use hello::method1;
use rand::random_range;
fn main() {
    method1();
    let x = random_range(0.0..=1e9);
    println!("{}", x);
}