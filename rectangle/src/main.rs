fn main() {
    let rectangle = Rectangle { width: 4, height: 3 };

    println!("{}", rectangle.to_string());
    println!("The area of the rectanle is {}", rectangle.area());
    println!("The figure is closed? {}", rectangle.is_close());
}

struct Rectangle {
    width: u32,
    height: u32,
}

// methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// traits
impl ToString for Rectangle {
    fn to_string(&self) -> String {
        format!("Rectangle:<{} x {}>", self.width, self.height)
    }
}

trait Geometry {
    fn is_close(&self) -> bool;
}

impl Geometry for Rectangle {
    fn is_close(&self) -> bool {
        true
    }
}

// y = kx + b
// struct Line {
//     k: f32,
//     b: f32
// }
//
// impl Geometry for Line {
//     fn is_close(&self) -> bool {
//         false
//     }
// }
