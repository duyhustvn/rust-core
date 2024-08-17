fn main() {
    let mut x = 5;
    println!("x: {x}");
    x = 6;
    println!("x: {x}");

    let x = "hi";
    println!("x: {x}");
    // x = 8; x cannot be assigned new value because it is mutable
    // println!("x: {x}");

    let x = 8;
    println!("x: {x}");

    // shadow the type of variable
    let x = "hello";
    println!("x: {x}");

    let mut x = 9;
    println!("x: {x}");
    // x = "how are you"; cannot mutate the type
    x = 10;

    another_function(x);

    let x = plus_one(x);
    println!("x: {x}");

    let x = plus_two(x);
    println!("x: {x}");

    let (a, b) = return_multiple();
    println!("a: {a} b: {b}");

    let result = return_multiple();
    println!("a: {} b: {}", result.0, result.1);

    loop_fn();
    for_fn();
}

fn another_function(x: i32) {
    println!("the value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    return x + 1;
}

fn return_multiple() -> (i32, char) {
    return (1, 'c');
}

fn loop_fn() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn for_fn() {
    let a = [10, 20, 30];
    for element in a {
        print!("{element} ");
    }
    println!();
}
