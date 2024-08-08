mod scan;
use std::io;

use scan::scan;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens = scan(&input);

    println!("{:?}", tokens);
}
