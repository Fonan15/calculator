use std::io;

fn main() {
    println!("Test");
    let x = read_var('x');
    let y = read_var('y');
    println!("Die Zahlen sind {} und {}",x,y);
    let mut z = plus(x, y);
    println!("Ergebnis: {}", z);
    z = if z != (minus(x,y)) {minus(x,y)} else {z};
    println!("Ergebnis nun {}", z);
    z = division(x, y);
    println!("Ergebnis nun {}", z);
    z = multiplikation(x, y);
    println!("Ergebnis nun {}", z);
}

fn read_var (c:char) -> i32{
    println!("Please type {}", c); 
    let mut v = String::new();

    io::stdin().read_line(&mut v)
    	.expect("Failed to read line!");

    let v: i32 = match v.trim().parse() {
            Ok(num) => num,
            Err(_) => return 0,
        };
//    let v: i32 = v.trim().parse()
//    	.expect("Please type a number!");

    return v;
}

fn plus(x:i32, y:i32) -> i32{
    println!("Summe: {}", x+y);
//    let z = x + y;
//    println!("Ergebnis: {}", z);
    return x+y;
}

fn minus(x:i32, y:i32) -> i32{
    println!("Subtraktion: {}", x-y);
    return x-y;
}

fn division(x:i32, y:i32) -> i32{
    println!("Division: {}", x/y);
    return x/y;
}

fn multiplikation(x:i32, y:i32) ->i32{
    println!("Multiplikation: {}", x*y);
    return x*y;
}


