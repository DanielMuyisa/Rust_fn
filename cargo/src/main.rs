use std::io;

fn main() {
    println!("Saisir un nombre");
    let mut nbr = String::new();

    io::stdin()
        .read_line(&mut nbr)
        .expect("Fsil to read the number");

    println! {"The number you've tap is {nbr}"}
}
