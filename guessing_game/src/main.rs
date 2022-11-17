use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guess {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            },
        } 
    }
    
}