fn main () {
    // If Expressions

    // Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions
    // The condiiton in the if statement MUST always be a boolean expression or boolean. Unlike JS, Rust will not automatically try to convert non-boolean types to a boolean. You must be explicit in rust
    let number = 3;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // else if expressions

    // You can use multiple conditions by cominbing if and else in a else if expression
    // Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    // If you have too many else if statements -> use match expressions (another "branching" construct in Rust)

    if number % 4 == 0 {
        println!("number divisibile by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else {
        println!("number is not divisible by 3 or 4");
    }

    // using if in a let statement

    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions (thats how we can use { 5 } etc in the let staetment below)
    // In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type (in our case i32). If mismatched types = you woill get an error

    let condition = true;
    let number_two = if condition { 5 } else { 6 };
    println!("The value of number is: {number_two}");
    // let wrong_number = if condition { 5 } else { "blah" }

    // The expression in the above if block evaluates to an integer, and the expression in the else block evaluates to a string. This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively. Knowing the type of number lets the compiler verify the type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

}