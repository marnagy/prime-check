use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<i64>() {
            Ok(number) => {
                if number >= 2 {
                    let mut is_prime: bool = true;
                    let mut divisor: i64 = -1;
                    println!("Sqrt of {0} is {1}.", number, (number as f32).sqrt());
                    //println!("Sqrt of {0} is {1}.", number, (number as f32).sqrt() as i32);
                    for n in 2..((number as f64).sqrt() as i64 + 1) {
                        if number % n == 0 {
                            is_prime = false;
                            divisor = n;
                            break;
                        }
                    }
                    if is_prime {
                        println!("{0} is a prime.", number);
                    } else {
                        println!("{0} is NOT a prime.", number);
                        println!("Divisible by {0}.", divisor);
                    }
                } else {
                    println!("Not a valid number.")
                }
            }
            Err(_) => println!("Not a valid number."),
        },
        Err(_) => println!("No input detected."),
    }
}
