use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io,
};

use rand::Rng;

fn main() {
    let mut input = String::new();
    
    
    'main: loop {
        println!("Guess the number");
        
        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            input.clear();

            println!("Please input your guess or type 'exit' to quit");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");


            match input.trim().to_lowercase().as_str() {
                "exit" => break 'main,                
                _ => (),
            };

            let parsed = match input.trim().parse::<u32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("{} isn't a number", input.trim());
                    continue;
                }
            };

            match parsed.cmp(&secret_number) {
                Less => println!("Too small!"),
                Greater => println!("Too big!"),
                Equal => {
                    println!("You win!");
                    break;
                }
            }
        }

        'play_again: loop {
            input.clear();

            println!("Play again? [y/N] ");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().to_lowercase().as_str() {
                "y" => continue 'main,
                "n" => break 'main,
                _ => continue 'play_again,
            }
        }
    }
}
