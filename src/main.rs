//imports module
mod args;
//uses module
use args::Args;

fn main() {
    let args = Args {
        image_1: String::new(),
        image_2: String::new(),
        output: String::new(),
    };
    println!("Hello, world!");
}

//learn about structs and iterators
