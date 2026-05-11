```rust

// x if x 
match n {
    n if n > 0 => "positif",
    n if n < 0 => "négatif",
    _ => "zéro",
}

// macro match!
matches!(self, DayOfWeek::Saturday | DayOfWeek::Sunday)


// Cow
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[ind] = -value;
        }
    }
}