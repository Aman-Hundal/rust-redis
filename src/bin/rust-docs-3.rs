fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is {x}");
    x = 10;
    println!("The value of x is {x}");

    // Shadowing
    // you can declare a new variable with the same name as a previous variable. 
    // Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see 
    // when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to 
    // itself until either it itself is shadowed or the scope ends.
    let y = 6;
    let y = y + 2;
    // below is scoped
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // y = 16
    }

    print1ln!("the value of y is {y}") // y =  8

    // Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few 
    // transformations on a value but have the variable be immutable after those transformations have been completed.
    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, 
    // we can change the type of the value but reuse the same name.
}