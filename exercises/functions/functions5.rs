// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3); // statement
    println!("The square of 3 is {}", answer); // statement
}

fn square(num: i32) -> i32 {
    num * num // expression (evaluates to a value)
}
