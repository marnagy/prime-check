use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<i64>() {
            Ok(number) => {
                is_prime(number);
            }
            Err(_) => println!("Not a valid number."),
        },
        Err(_) => println!("No input detected."),
    }
}

fn is_prime(number : i64) -> Result<bool, String> {
    if number >= 2 {
        let mut is_prime = true;
        let mut divisor: i64 = -1;
        println!("Sqrt of {0} is {1}.", number, (number as f32).sqrt());
        //println!("Sqrt of {0} is {1}.", number, (number as f32).sqrt() as i32);
        for n in 2..((number as f64).sqrt().floor() as i64 + 1) {
            if number % n == 0 {
                is_prime = false;
                divisor = n;
                break;
            }
        }
        return Result::Ok(is_prime);
    } else {
        return Result::Err( String::from("Not a valid number"));
    }
}