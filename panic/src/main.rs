use std::fs::File;

fn main() {
    // let f = File::open("hello.txt");

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("File not found!");
    //     }
    // };


    // Use unwrap()
    // let f = File::open("hello.txt").unwrap();

    // Use expect()
    let f = File::open("hello.txt").expect("We don't have file yet");
}
