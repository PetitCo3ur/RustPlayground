use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {query} in {filename}");

    let content = std::fs::read_to_string(filename).expect("Could not read the file");
    
    println!("With text:\n{content}");
}
