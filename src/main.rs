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

        let input_ok: u32 = match input_str.trim().parse() {
            Ok(input_ok) => input_ok,
            Err(_) => {
                println!(
                    "The number {:?} you input is not a number.",
                    input_str.trim()
                );
                continue;
            }
        };

        match input_ok.cmp(&random_number) {
            Ordering::Less => println!("The number {} you guess is too small.", input_ok),
            Ordering::Greater => println!("The number {} you guess is too big.", input_ok),
            Ordering::Equal => {
                println!("The number {} you guess is correct.", input_ok);
                break;
            }
        }
    }
}
