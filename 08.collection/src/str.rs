pub fn str() {
    let data = "initial contents";

    let s = data.to_string(); // convert str to String
    println!("s: {s}");

    // the method also works on a literal directly
    let s = "initial contents".to_string();
    println!("s: {s}");

    let s = String::from("initial contents");
    println!("s: {s}");
    // s.push_str("hello"); // cannot update immutable string

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str take reference of string not ownership
    println!("s2: {s2}");
    println!("s1: {s1}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s: {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // s1 must be taken ownership and other is reference
    // let s3 = s1 + s2;
    // let s3 = &s1 + s2;
    // let s3 = &s1 + &s2;
    println!("s3: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");

    let s = String::from("Зд");
    for c in s.chars() {
        print!("{c} ");
    }
    println!("");

    for b in s.bytes() {
        print!("{b} ");
    }
    println!("");
}
