pub fn local_main() {
    println!("REFERENCES AND BORROWING");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s1: {s1} len: {len}");

    cannot_modify_immut_reference(&s1);

    let mut s2 = String::from("hello");
    modify_mut_reference(&mut s2);
    println!("s2: {s2}");

    invalid_mut_immut_reference_at_the_same_time();
    valid_mut_immut_reference_at_the_same_time();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn cannot_modify_immut_reference(s: &String) {
    // s.push_str(" world"); ERROR
    println!("s: {s}");
}

fn modify_mut_reference(s: &mut String) {
    s.push_str(" word.");
}

fn invalid_mut_immut_reference_at_the_same_time() {
    let mut s1 = String::from("hello");

    let s2 = &s1;
    // let s3 = &mut s1; // invalid cannot have mutable and immutable reference at the same time
    // println!("s2: {s2} s3: {s3}");
}

fn valid_mut_immut_reference_at_the_same_time() {
    // At any given time, you can have either one mutable reference
    // or any number of immutable references.
    let mut s1 = String::from("hello");

    let s2 = &s1;
    let s3 = &s1;
    println!("s2: {s2} s3: {s3}");
    // variables s2, s3 will not be used after this point

    let s4 = &mut s1;
    println!("s4: {s4}");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
 */
