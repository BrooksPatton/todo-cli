use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("{:?}", arguments);
}
