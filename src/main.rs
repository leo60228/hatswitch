use std::env;
use std::fs;

fn main() {
    let file = env::args().nth(1).unwrap();
    let data = fs::read(file).unwrap();
    println!("{:#x?}", hatswitch::gamestate(&data));
}