use std::io;
use std::cmp::Ordering;

fn main() {
    let my_num = rand::random::<u8>();
    let mut user_guess_num: u8;
    let mut user_guess;

    loop {
        
        user_guess = String::new();
        println!("Guess the number!");
        
        io::stdin().read_line(&mut user_guess)
            .expect("Failed to read line!");
        
        user_guess_num = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers");
                continue;
            }
        };
        
        match user_guess_num.cmp(&my_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("My number is : {}", my_num);
}
