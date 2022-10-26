fn main() {
    // 1
    // let y = {
    //     let x = 10;
    //     x + 2
    // };

    // let y = five(1);
    // println!("The value of y is: {y}");

    // controlFlow(5);

    // loops();

    loopsImpr();
}

fn five(x: i32) -> i32 {
    5 + x
}

// control flow
fn controlFlow(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loops() {
    let mut counter = 0;
    let increment = -1;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 8;
        }
    };

    println!("The result is {result}");
}

fn loopsImpr() {
    let mut count = 0;
    'counting_up: loop {
        count += 1;

        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }

            if count == 10 {
                break 'counting_up;
            }
            remaining -= 2;
        }
    }
    println!("end of loop : {count}");
}

fn function_name() {
    // function body
}

fn get_pi() -> f64 {
    22.0 / 7.0
}
