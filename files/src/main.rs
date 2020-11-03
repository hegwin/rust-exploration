use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // for arg in args.iter() {
    //     println!("{}", arg)
    // }
    //
    println!("{}", args[1]);
    let file_path = &args[1];
    let mut file = File::open(&file_path).expect("Can't open file!");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Oops cant't read file");
    println!("File contents: \n\n{}", contents);

    let mut output_file = File::create("output.txt").expect("Can't create file");
    output_file.write_all(b"Welcome to Hell!").expect("Can't write to file");
}
