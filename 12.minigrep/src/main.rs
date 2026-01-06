use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if (args.len() != 3) {
        panic!("Syntax: cargo run -- [search_string] [file]");
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("Search for {query}");
    println!("In file {file_path}");
}
