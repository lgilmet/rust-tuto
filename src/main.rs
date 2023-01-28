#![allow(unused_imports)]
// TODO: Remove this line when you're done. This is just to suppress warnings for unused imports in the console.

use rand::Rng;
use std::cmp::Ordering;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};

// where should i create functions?
// you can create functions anywhere in your code. It's best to create them in the same file where you're going to use them.
fn main() {
    // loop through an array and get the length, min and max
    // let arr = [12, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut min = arr[0];
    // let mut max = arr[0];
    // let mut sum = 0;
    // // for i in arr.iter() {
    // //     if i < &min {
    // //         min = *i;
    // //     }
    // //     if i > &max {
    // //         max = *i;
    // //     }
    // //     sum += i;
    // // }
    // // use loop instead
    // let mut i = 0;
    // loop {
    //     if i >= arr.len() {
    //         break;
    //     }
    //     if arr[i] < min {
    //         min = arr[i];
    //     }
    //     if arr[i] > max {
    //         max = arr[i];
    //     }
    //     sum += arr[i];
    //     i += 1;
    // }
    // println!("Min: {}", min);
    // println!("Max: {}", max);
    // println!("Sum: {}", sum);
    // println!("Length: {}", arr.len());

    /*
    // show me something else
    // sure, here's a simple calculator.
    let mut num_1 = String::new();
    let mut num_2 = String::new();
    let mut operator = String::new();
    println!("Enter first number:");
    io::stdin()
        .read_line(&mut num_1)
        .expect("Failed to read line");
    println!("Enter second number:");
    io::stdin()
        .read_line(&mut num_2)
        .expect("Failed to read line");
    println!("Enter operator:");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let num_1: f32 = num_1.trim().parse().expect("Failed to parse number");
    let num_2: f32 = num_2.trim().parse().expect("Failed to parse number");
    let operator: char = operator.trim().parse().expect("Failed to parse operator");
    let result: f32 = match operator {
        '+' => num_1 + num_2,
        '-' => num_1 - num_2,
        '*' => num_1 * num_2,
        '/' => num_1 / num_2,
        _ => panic!("Invalid operator"),
    };
    println!("{} {} {} = {}", num_1, operator, num_2, result);
     */

    // Functions
    // print_hello("John");

    // // FILE I/O
    // let mut file = File::open("src/main.rs").expect("Failed to open file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Failed to read file");
    // println!("File contents: {}", contents);

    /*
    // Guessing Game
    let random_num = rand::thread_rng().gen_range(1..101); // 1 to 100
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&random_num) {
            // whats the cmp method?
            // cmp method compares two values and can be used with any type that implements the PartialOrd trait.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }*/

    /*
    // MATH OPERATIONS
    let num_1 = 3;
    let num_2 = 7;
    println!("{} + {} = {}", num_1, num_2, num_1 + num_2);
    println!("{} - {} = {}", num_1, num_2, num_1 - num_2);
    println!("{} * {} = {}", num_1, num_2, num_1 * num_2);
    println!("{} / {} = {}", num_1, num_2, num_1 / num_2); // 0
                                                           // why is the result 0?
                                                           // because num_1 and num_2 are integers. If you want to get a decimal result, you need to convert one of the numbers to a float.
    println!("{} / {} = {}", num_1, num_2, num_1 as f32 / num_2 as f32);
    println!("{} % {} = {}", num_1, num_2, num_1 % num_2);
    println!("{} ^ {} = {}", num_1, num_2, num_1 ^ num_2);
    // what is ^ operator?
    // ^ is the bitwise XOR operator. It compares the bits of two numbers and returns 1 if the bits are different.
    println!(
        "{}.pow({}) = {}",
        num_1,
        num_2,
        (num_1 as u32).pow(num_2 as u32)
    );
    println!("{} << {} = {}", num_1, num_2, num_1 << num_2);
    println!("{} >> {} = {}", num_1, num_2, num_1 >> num_2);
    // what are << and >> operators?
    // << is left shift and >> is right shift. They shift the bits of the number to the left or right.
    println!("{} & {} = {}", num_1, num_2, num_1 & num_2);
    */

    // DATA TYPES
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    // Signed integers: i8, i16, i32, i64, i128, isize
    // println!("Max u32: {}", u32::MAX);
    // println!("Max i32: {}", i32::MAX);
    // println!("Max f32: {}", f32::MAX);
    // println!("Max f64: {}", f64::MAX);
    // println!("Max char: {}", char::MAX);
    // println!("Max usize: {}", usize::MAX);
    // what is usize?
    // usize is the number of bytes it takes to reference any location in memory.

    // let bool_value = true;
    // let char_value = 'a';
    // let float_value = 2.0; // f64
    // let int_value = 2; // i32
    // let string_value = "Hello, world!";
    // let array_value = [1, 2, 3];
    // let tuple_value = (1, 2, 3);
    // let num_value: f32 = 2.111111111111;
    // println!("f32: {}", num_value); // 2.1111112
    // let num_value: f64 = 2.1111111111111111111111;
    // println!("f64: {}", num_value); // 2.111111111111111
    // println!(
    //     "{} {} {} {} {} {} {}",
    //     bool_value, char_value, float_value, int_value, string_value, array_value[0], tuple_value.0
    // );
    // does rust support ++ and --?
    // no, rust does not support ++ and --. Instead, you can use += 1 or -= 1.

    // NUMBERS
    // const MAX_GUESSES: u32 = 5_000;
    // const PI: f64 = 3.14159265359;
    // let age: &str = "33";
    // let mut age: u32 = age.trim().parse().expect("Please type a number!");
    // age = age + 1;
    // println!("Your age is {}, and pi is {}", age - 1, PI);

    // FIRST EXAMPLE
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");
    // println!("Hello, {}! {}", name.trim_end(), greeting);
}

// create a function here to print hello with a name as a parameter
// fn print_hello(name: &str) {
//     println!("Hello, {}!", name);
// }
