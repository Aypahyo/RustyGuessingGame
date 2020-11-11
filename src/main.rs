use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // resets buffer every time be re-declaring inside the loop
        let mut guess = String::new();

        // https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        // expect == unwrap with a message
        // let guess_as_int: u32 = guess.trim().parse().expect("Please type a number!");
        let guess_as_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_as_int.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
