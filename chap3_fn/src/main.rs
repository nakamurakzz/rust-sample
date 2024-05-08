fn main() {
    println!("Hello, world!");
    another_function();
    print_int(22);
    print_int_float(11, 1.11);
    let x = five();
    println!("five: {x}");
    let y = plus_one(x);
    println!("six: {y}");
}

fn another_function() {
    println!("Hello Hello");
}

fn print_int(x: i32) {
    println!("x: {x}");
}

fn print_int_float(x: i32, y: f64) {
    println!("x: {x}, y: {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
