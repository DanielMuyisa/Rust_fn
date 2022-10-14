use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter a number");
    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // convert
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You've enter {input}");
        println!("the random gen is {}", random);

        // comparison

        match input.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        println!("=========================");
        println!("Enter a other number");
    }
}
