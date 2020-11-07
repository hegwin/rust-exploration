use std::collections::HashMap;

fn main() {
    let mut albums = HashMap::new();

    albums.insert("Lady Gaga", "The Fame");
    albums.insert("Tylor Swift", "Red");

    println!("{:?}", albums);


    for (k, v) in albums.iter() {
        println!("{} => {:?}", k, v);
    }

    match albums.get("Lady Gaga") {
        Some(v) => println!("Here is the album: {}", v) ,
        None => println!("No data bitch! Try again later.")
    }

    // Remove a value
    // albums.remove("Tylor Swift");
    // println!("{:?}", albums);

    // Check a value
    if albums.contains_key("Lady Gaga") {
        println!("Here you go!");
    } else {
        println!("Hey man! It's impossible!");
    }
}
