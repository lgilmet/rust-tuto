#![allow(unused_imports)] // TODO: Remove this line when you're done. This is just to suppress warnings for unused imports in the console.

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const MAX_GUESSES: u32 = 5_000;
    const PI: f64 = 3.14159265359;
    let age: &str = "33";
    let mut age: u32 = age.trim().parse().expect("Please type a number!");
    age = age + 1;
    println!("Your age is {}, and pi is {}", age - 1, PI);

    // FIRST EXAMPLE
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");
    // println!("Hello, {}! {}", name.trim_end(), greeting);
}
