fn main() {
    let x = 2;
    // x = x/2; -> cannot assign twice to immutable variable
    // Each time we reuse a variable name, we are creating a brand-new variable, not modifying the previous one.
    // We call this shadowing: the new variable with the same name shadows the old variable, 
    // so you can’t access the old one anymore
    // to fix it:
    // let mut x = 2;
    // x = x/2;
    let x = x/2;
    println!("x: {}", x);
}