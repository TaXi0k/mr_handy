use mr_handy::{self, confirm, input, pause, printf};

fn main() {
    printf!("Input: {}", input!("user# "));
    pause();
    println!("Hello world!");
    println!("{}", confirm("Confirm please", Some(true)));
}