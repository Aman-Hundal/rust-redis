fn main () {
    println!("Hello, world!");
    // rust uses snake_case for function names and variables. functions are declared using the fn keyword followed by the function name and a set of parentheses.
    // call a functions like below

    // we are using a return value of a fn to insitalize a varaiblae (and making a statement using an expression)
    let res: i32 = add(1,3);
    println!("RES {res}");

    // Note that we defined add after the main function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller
    // We can define functions to have parameters, which are special variables that are part of a function’s signature.
    // In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design:

    // Statements and Expressions -> Function bodies are made up of a series of statements optionally ending in an expression
    // Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions
    // Statements -> are instructions that perform some action and do not return a value (therefore you can’t assign a let statement to another variable,)
    // Expressions -> evalualte to a resultant value. Evalualte to a value and make up most of the rest of the code that youll write in rust. ex (5+6)
    // Expressions can be part of statements (ie let x = 6 || 6 is the expression that evalulates to the alue of 6)
    //  Calling a function is an epxression. Calling a Macros is an expression. A new scope block created by with {} is an expression

    // Block Expression Example
    // the {} code is the expression -> let ex is a statemnt,let x = 3 is another statment and x+1 is a expression
    // BIG NOTE -> EXPRESSIONS DO NOT INCLUDE ; at the end. If you do this you turn the epxression into a statement. If you added ; at end of x + 1 your let ex = {} would return no value . RETURN is an exception to this rule.
    let ex = {
        let x = 3;
        x + 1;
    };

    // Functions can return values to the code that calls them. We must declare their type after an arrow (->). the return value is sysnomous with teh value of the final expression in teh block of the body of a fn


}

// Functions definitons are statements themselves. Calling functions is not a statement though. Calling a function is an epxression.
fn add (n1: i32, n2: i32) -> i32 {
    //Statement
    let x = n1 + n2;
    // println!("{x}");
    // Expressions dont have ; BUT RETURN is an exception to this rule. return calls at end of fn's are like statements and therefore need a ; even though they are expressions that return a value. The return keyword explicitly specifies the value to return from a function, and it requires a semicolon because it is a statement.

    // you can remove the return and ; and just have x at the end of this fn to do the same thing. But idiomatic Rust typically avoids return unless necessary.
    return x;
}