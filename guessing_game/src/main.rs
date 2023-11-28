// importing a external library or crate
extern crate rand;

// random number generator require Rng train to be in scope which defines the random number generator
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    // the thread_rng gives us a copy of random number that is local to current thread of execution and seeded by os
    // gen_range takes two nums start and stop, generates a number inclusive of start and exclusive of end
    // cargo doc --open 
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("secret number {}", secret_number);
    println!("Guess the number!");
    // loop introduces a infinite loop of code 
    loop {
        // by default the variables in rust are immutable
        // we have to use **mut** before the variable name to make it mutable as shown below
        let mut guess = String::new(); // string::new creates a new instance of string type provided by std library which is growable
        // the new function is used to create a new instance of a type and is used in many types
        // the `::` is used to indicate the function is associated with respective type(class) and we call them classmethods in python and static methods in others
        println!("Please input your guess");
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("You guessed: {}", guess);

        // converting string guess to int guess
        // rust allows to shadows the prev defined str variable value of guess with new one
        // if we use expect the program will crash incase of any invalid input, we can handle the Result type of parse function

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        // if parse is success it return the result with Ok arm that cotain the number 
        // err contain more info about error but we are not handling it, _ catches all error types
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // a match is an expression made of arms, an arm contains a pattern and the code that need to be run if value given to match is matching 
        // the arms pattern
        match guess.cmp(& secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("yay you guess the right number");
                break;
            }
        }
    }

}