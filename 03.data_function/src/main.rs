fn main() {
    let mut x = 5;
    println!("x: {x}");
    x = 6;
    println!("x: {x}");

    // shadow the type of variable
    let x = "hi";
    println!("x: {x}");
    // x = "hello"; x cannot be assigned new value because it is mutable

    // shadow the type of variable
    let x = "hello";
    println!("x: {x}");

    let mut x = 9;
    println!("x: {x}");
    // x = "how are you"; cannot mutate the type
}
