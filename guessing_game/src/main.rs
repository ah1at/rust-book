use std::io; // a library that allows you to obtain user input and print the result
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // a random number generator that's local to the current thread and seeded by the OS
    // range 1..101 is inclusive at lower bound, exclusive at upper bound
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");

        // a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // takes stdin and appends that to the string given in the arg
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // breaks loop
            }
        }
    }
}