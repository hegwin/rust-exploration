fn main() {
    let language = "Ruby";

    println!("How do {} programmers call themselves? {}", &language, match get_profession(&language) {
        Some(profession) => profession.to_string(),
        None => "Unknown".to_string()
    });
}

fn get_profession(name: &str) -> Option<&str> {
    match name {
        "Ruby" => Some("Rubyist"),
        "Rust" => Some("Rustacean"),
        "Elixr" => Some("Alchmeist"),
        _ => None
    }
}
