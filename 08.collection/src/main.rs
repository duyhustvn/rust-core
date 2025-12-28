fn main() {
    let immutable_vector = vec![1, 2, 3];
    // immutable_vector.push(5); cannot change immutable vector
    for v in immutable_vector {
        print!("{} ", v);
    }
    println!("");

    let mut mutable_vector_1 = Vec::new();
    mutable_vector_1.push(1);
    mutable_vector_1.push(2);
    mutable_vector_1.push(3);
    for v in &mutable_vector_1 {
        print!("{} ", v);
    }
    println!("");
    let first = mutable_vector_1[0]; // it is just a copy
    println!("first: {}", first);

    let second = &mutable_vector_1[1]; // immutable borrow
    println!("second: {}", second);

    // mutable_vector.push(4); // will cause panic cause this is mutable borrow, cannot have
    // immutable and mutable borrow at the same time

    let mut mutable_vector_2 = vec![1, 2, 3];
    for v in &mutable_vector_2 {
        print!("{} ", v);
    }
    println!("");

    let first = &mut mutable_vector_2[0];
    *first += 50;

    for v in &mutable_vector_2 {
        print!("{} ", v);
    }
    println!("");
}
