fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);
    let a = sum(10, 5);
    println!("sum = {}", a);

    let f = fact(10);
    println!("Fact(10) = {}", f);
}

fn another_function() {
    println!("Another function");
}


fn fact(n:i32) -> i32 {
    if n == 1 {
        1
    } else {
        n * fact(n - 1)
    }
}


fn another_function2(x:i32) {
    println!("The value of x = {}", x);
}

fn sum(x:i32, y:i32) -> i32 {
    x + y
}
