fn main() {
    // println!("Hello, world!");
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    //let s1 = "Hello";
    let s2 = s1;

    // error[E0382]: borrow of moved value: `s1`
    println!("{}", s1);
}
