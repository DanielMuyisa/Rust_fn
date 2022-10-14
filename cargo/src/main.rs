use std::io;

fn main() {
    let float = 4.5;
    let float: f32 = 40.1;
    // println!("{float}");

    operations();

    // Bool with explicit type
    let bl = true;
    let bl: bool = false;
    // println!("{bl}");

    // ------ TUPLE
    let tuple: (i32, f64, u8) = (400, 5.3, 1);
    let (x, y, z) = tuple;

    // println!("the value of z is {z}");
    // println!("{}", { tuple.0 });

    // ------TABLE
    let table = [1, 2, 3, 4, 5];
    // presise length
    let tab: [i32; 3] = [1, 2, 3];
    let months = [
        "Janvier", "Fevrier", "Mars", "Avril", " Mais", "June", "July",
    ];
    println!("{}", months[0]);

    println!("======== ENTER A NUMBER =========");

    // saisir
    let mut input = String::new();
    // save value
    io::stdin().read_line(&mut input).expect("Valeur invalide");
    // convert

    let userValue: usize = input.trim().parse().expect("Value must be intege");
    if (userValue > 0 && userValue < 8) {
        println!("Conresponded value is {}", months[userValue - 1]);
    } else {
        println!("Non prise en charge");
    }
}

fn operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // println!("{remainder}, {quotient} ");
}
