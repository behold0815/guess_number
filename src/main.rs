use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("==== Guess number game. ====");
    let random_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a number: ");
        let mut input_str = String::new();

        stdin()
            .read_line(&mut input_str)
            .expect("The number you input is unreadable.");

        let input_str: u32 = match input_str.trim().parse() {
            Ok(input_str) => input_str,
            Err(_) => {
                println!(
                    "The number {:?} you input is not a number.",
                    input_str.trim()
                );
                continue;
            }
        };

        match input_str.cmp(&random_number) {
            Ordering::Less => println!("The number {} you guess is too small.", input_str),
            Ordering::Greater => println!("The number {} you guess is too big.", input_str),
            Ordering::Equal => {
                println!("The number {} you guess is correct.", input_str);
                break;
            }
        }
    }
}
