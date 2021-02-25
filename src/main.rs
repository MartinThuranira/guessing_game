fn main() {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;
    loop {
        let mut guess = String::new();
        println!("Type in your guess below, a number between 1-10");
        io::stdin().read_line(&mut guess).expect("Failed to read!");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        let secret_number = rand::thread_rng().gen_range(1, 11);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("The number you guessed was too small");
                println!("The correct number is: {}", secret_number);
                println!("Your guess was {}", guess);
            }
            Ordering::Equal => {
                println!("You got it right!!");
                break;
            }
            Ordering::Greater => {
                println!("The number you guessed was too big");
                println!("The correct number is: {}", secret_number);
                println!("Your guess was {}", guess);
            }
        }
    }
}
