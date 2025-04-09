use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // rand::thread_rng() is a fn that creates a specific random number generator
    let secret_number = rand::thread_rng()
        // This method is defined by the Rng trait that we brought into scope with the use rand::Rng;
        .gen_range(1..=10);
    loop {
        println!("Please enter a guess");
        // LET, MUT and & is VERY IMPORTANT TO RUSTS OWNERSHIP RULES (and theory on pointers, references, mutability, immutatbility etc)
        let mut guess = String::new(); // example of type inference and we didnt have to say what type guess was

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

        // Shadowing (allows you to rename a var and more)
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess,
        // but for now, know that this feature is often used when you want to convert a value from one type to another type.
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a valid number.");
        
        // use a match expression (AND NOT EXPECT CALL and crash program) to handle error and allow user to keep guessing until a VALID NUMBER is provided and not crassh.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch all value to catch all types of errors and to do something after (which is to continue and go to the NEXT ITTERATION OF THE LOOP)
        };
        
        println!("You guessed: {guess}");

        // match is a rust expression. Cmp, Ordering come from the above use std::cmp:Ordering
        // We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number
        // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
        // Rust takes the value given to match and looks through each arm’s pattern in turn. 
        match guess.cmp(&secret_number) { // Returns Ordering ENUM with VARIANT GREATER (Ordering::Greater) which is passed to the match expression and goes through the arms below
            Ordering::Less => println!("Your guess was too low!"), // arm
            Ordering::Greater => println!("Your guess was too high!"), // arm
            Ordering::Equal => {
                println!("Winner!");
                //BREAK STATEMENT TO STOP LOOP IF CORRECT ANSWER
                break;
            } // arm
        }
    }
    println!("The secret number was {secret_number}");
    // VERY HELPFUL CARGO FN = cargo doc --open -> LINKS YOU TO ALL THE DOCUMENTAITON FOR YOUR DEPENDANCIES IN A BROWSER
    // CONTINUE TO GO TO RUST BOOK CHAPTER 6
}
