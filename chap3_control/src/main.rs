fn main() {
    let x = 3;
    if x > 5 {
        println!("x is more than 5.")
    } else {
        println!("x is less than or equal to 5.")
    }

    let mut y = 1;
    let result = loop {
        y += 1;
        println!("y: {y}");
        if y == 10 {
            break y;
        }
    };

    println!("result: {}", result);

    let mut c = 0;
    'count: loop {
        let mut d = 10;
        loop {
            println!("d: {}", d);
            println!("c: {}", c);
            if d == 9 {
                break;
            }
            if c == 2 {
                break 'count;
            }
            d -= 1;
        }
        c += 1;
    }

    let mut e = 5;
    while e != 0 {
        println!("e: {}", e);
        e -= 1;
    }

    let arr = [1, 10, 100, 1000, 10000];
    for elm in arr {
        println!("elm: {}", elm);
    }
}
