fn main() {
    let y = {
        five() + 1
    };

    println!("The value of y is: {y}");
}

fn five () -> i32 {
    5
}