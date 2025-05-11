fn main () {
    // Loops. Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning (TYPEs = loop, while, and for)

    // loop key word tells Rust to execucete a block of code ove and over again forever or until you explicitly tell it to stop
    // You can place the break keyword within the loop to tell the program when to stop executing the loop
    // You can also use continue, which tells the program/loop to skip over any remaining code in thhis itteration of the loop and go to the next itteration.

    loop {
        println!("BANG");
        break;
    }
    
    // returning values from loops
    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it
    // return vs break = break only exits the current loop, return always exits the current function. Can use either in a loop

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10;
        }
    };
    println!("Result: {result}");

    // RUST NOTE -> If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loops. A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. You can do this with using a loop but it requires a lot of code. Its such a common thing that rust has a built-in language construct for it called a while loop.

    // let mut number_three = 3;
    // while number_three != 0 {
    //     println!("{number_three}");
    //     number_three -= 1;
    // }
    // println!("LIFTOFF");

    // For loops. Used to loop through collections.
    // The safety and conciseness of for loops make them the most commonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop.
    let a = [10,20,30,40,50];
    for elm in a {
        println!("the value of the elm: {elm}");
    }

    // You can use a Range provided by the Standard Library which generated number in seq starting from one number and ending before another number. WE will do that here to recreate the while loop liftoff example
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF");


}