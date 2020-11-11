use std::process::Command;

fn main() {
    // `ruby hello.rb`
    let mut cmd = Command::new("ruby");
    cmd.arg("hell.rb");

    // execution
    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout))
            }
        },
        Err(e) => println!("Error: {}", e)
    }
}
