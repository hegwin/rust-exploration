#![allow(dead_code)]

enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("I'm inside call!")
    }
}

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday,
    Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => false,
            _ => true
        }
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();

    let day = Day::Sunday;
    println!("Is the day a weekday? {}", day.is_weekday());

    let day = Day::Tuesday;
    println!("Is the day a weekday? {}", day.is_weekday());
}


