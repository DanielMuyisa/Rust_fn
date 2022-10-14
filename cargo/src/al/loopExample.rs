use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter a number");

    loop {
        let mut input = String::new();
        let random = rand::thread_rng().gen_range(1..=100);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You've enter {input}");
        println!("the random gen is {}", random);

        // comparison
        let input: u32 = input.trim().parse().expect("Please type a number!");

        match input.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        };

        println!("=========================");
        println!("Enter a other number");
    }
}
