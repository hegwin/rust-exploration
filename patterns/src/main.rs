fn main() {
    let name = String::from("Tom");

    println!("Character at Position 4 is: {}",
             match name.chars().nth(4) {
                 Some(c) => c.to_string(),
                 None => "No character found".to_string(),
             }
    )
}
