const CONST_TEST: u32 = 25 * 2;

fn main() {
    println!("CONST_TEST: {CONST_TEST}");
    let x = 5;
    println!("x: {x}");

    let x = 10;
    println!("x: {x}");

    // let y = "42".parse().expect("error"); // error
    let y: u32 = "42".parse().expect("error");
    println!("y: {y}");

    // tubple
    let tup: (i64, f64, char) = (24, 3.21, 'a');
    let (a, b, c) = tup;
    println!("a: {a}, b:{b}, c:{c}");
    println!("a: {}, b:{}, c:{}", tup.0, tup.1, tup.2);

    // array
    let ar: [i64; 2] = [1, 2];
    println!("{},{}", ar[0], ar[1])
}
