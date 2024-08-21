pub fn local_main() {
    println!("OWNERSHIP");
    move_value();
    clone_value();

    let s = String::from("how are you"); // s comes into scope
    take_ownership(s); // s's value moves into function
                       // println!("string: {s}"); s cannot be access from now
                       // because ownership is pass to take_ownership

    let x = 5; // x comes into scope
    makes_copy(x); // still can access x afterward, because x is copied, not move
    println!("x: {x}");

    let s1 = give_ownership();
    println!("s1: {s1}");

    let s2 = String::from("Take ownership and give back");
    let s3 = take_and_gives_back(s2);
    println!("s3: {s3}");
}

fn move_value() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");
    // println!("{s1}"); cannot print s1, s1 is moved to s2
}

fn clone_value() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {s1} s2: {s2}");
}

fn take_ownership(s: String) {
    println!("string: {s}");
}

fn makes_copy(i: i32) {
    println!("i: {i}");
}

fn give_ownership() -> String {
    let s = String::from("I give you the ownership");
    return s;
}

fn take_and_gives_back(s: String) -> String {
    return s;
}
