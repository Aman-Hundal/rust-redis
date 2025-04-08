use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guess = String::new();
    // rand::thread_rng() is a fn that creates a specific random number generator
    let secret_number = rand::thread_rng()
    // This method is defined by the Rng trait that we brought into scope with the use rand::Rng;
    .gen_range(1..=100);

    println!("Please enter a guess");

    io::stdin()
    // READLINE RETURNS RESULT TYPE WHICH HAS A .EXPECT METHOD TO ERROR HANDLE
    // If this instance of Result is an Err value, expect will cause the program to crash 
    // and display the message that you passed as an argument to expect. 
    // If this instance of Result is an Ok value, expect will take the return value that 
    // Ok is holding and return just that value to you so you can use it. 
    // In this case, that value is the number of bytes in the user’s input.
    // BEST PRACTICE TO HANDLE RESULT WOULD BE TO RUN ERROR HANDLING CODE AND NOT EXPECT
    .read_line(&mut guess)
    .expect("Failed to read line");

    // VERY HELPFUL CARGO FN = cargo doc --open -> LINKS YOU TO ALL THE DOCUMENTAITON FOR YOUR DEPENDANCIES IN A BROWSER
    println!("You guessed: {guess}");

    // match is a rust expression. Cmp, Ordering come from the above use std::cmp:Ordering
    // We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number
    // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
    // Rust takes the value given to match and looks through each arm’s pattern in turn. 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess was too small!");
        Ordering::Greater => println!("Your guess was too big!");
        Ordering::Equal => println!("Winner!");
    }

}
