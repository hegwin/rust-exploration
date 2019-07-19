fn main() {
    let tup:(i32, f64, u8) = (1989, 3.14, 8);

    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    let a = tup.0;
    let b = tup.0;

    println!("a = {}, b = {}", a, b);
}
