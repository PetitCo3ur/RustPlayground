```rust

// x if x 
match n {
    n if n > 0 => "positif",
    n if n < 0 => "négatif",
    _ => "zéro",
}

// macro match!
matches!(self, DayOfWeek::Saturday | DayOfWeek::Sunday)