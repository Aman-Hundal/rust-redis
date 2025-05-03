fn main() {
    // Every value in Rust is of a certain data type (subsets = 1. scalar and 2. compound)

    // Scalar types - 1. integer 2. floating point 3. boolean 4. character
    
    // Integer types - number without a fractional component. Signed and unsigned types that range from 8, 16, 32, 64, and 128 bits (and arch which depends on the architecture of your comp)
    // Signed and unsigned types refer to whether its possible for the number to be a negative (whether the number needs to have a sign with it - signed OR whether it will only be a positive - unsigned.)
    // Rust defaults to i32 integer types.
    // Integer formula Signed = (2^bits-1) to (2^bits-1)-1. Unsigned = 0 to (2^bits) - 1
    // Read on Rusts Integer Overflow
    let mut x: i32 = 28;

    // Floating Point Types - numbers with decimal points
    // Rust has two types for floating point types - f32 and f64 (32 bits and 64 bits). Default is f64
    // All floating poins are signed
    let mut y: f64 = 3.0;

    // Numeric Operations = addition (+), subtraction (-), multiplication (x), diision (/) and remainder (%)
    // Integer division truncates toward zero to the nearest integer
    
    // Boolean Types - true or false. 1byte in size. Denoted by bool. Rust can infer its a boolean though
    let t: bool = true;

    // Chracter Type - Char type is the languges most primative alphabetic type.
    // Note: we specify char literals with SINGLE QUOTES as opposed to string litterals which use double quotes
    // Char value is 4 bits (fixed size) in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII
    // A Char represents a SINGLE UNICODE SCALAR VALUE in single quotes
    // A String is a growable heap allocated sequence of characters. Dynamically sized (not fixed at 4 bits like char)
    let c = 'c'; // char inferred
    let z: char = 'ℤ';
    let name: &str = "Aman";

    // Compound Types - Primitive Types = 1.Tuple, 2. Array. Compound types can group multiple values into one type (tuple or array)
    
    //Tuple - is a general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a FIXED LENGTH  (once declared they cannot grow or shrink in size).
    // Tuples are created by writing comma seperated values in (). Each position in the tuple has a type and the types of the different values do not have to be the same
    let tup: (i32, f64, char, &str) = (500, 6.4, 'c', "Aman");
    // To get values of out of a tuple we can use pattern matching to destructure a tuple value
    let (x,y,z,g) = tup;
    println!("the value of y is {y}"); // will return 6.4
    //Directly accessing a tuple - using a period (.) followed by indexof the value (staring from 0)
    let value2 = tup.1;
    // Tuple without any values has a special name - unit. These units - REPRESENT AN EMPTY VALUE OR AN EMPTRY RETURN TYPE. Denoted with () . 
    // Expressions implicitly return the unit value if they don’t return any other value.

    // Array Type - A collecton of multiple values (like a tuple) BUT every element of an array has to the same type. Like Tuples, Arrays in Rust have a FIXED LENGTH
    // Explicitly defining a 5 length i32 array in Rust (RUST CAN INFER THIS AS WELL WITHTOUT THE TYPE)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // An array is a single chunk of memeory of a known fixed size that can be allocated on the stack. 
    // Arrays are usefule when you want your data allocated on the stack ( the same as the other types we have seen so far) or when you want to ensure you always hae a fixed number of elements
    // Arrays vs Vectors -> Vectors are a similar collection type povided by the standard rust lib, BUT ARE ALLOWED TO GROW OR SHRINK IN SIZE. More flexible than arrays.
    // Arrays are beter to use when you know the number of elements will not need to change (like if you are using a list of names of the months)
    // You can also iniatliaze an array to contain the same value for each element as follows
    let three_by_five = [5; 3];
    //Accessing Array elements = SAME As all other programing languages
    // Invalid array element acces -> RUNTIME error in RUST and the program will panic
    //This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed
    //Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing (BUT YOU HAVE TO WRITE BETTER CODE TO HANDLE THIS ERROR SAFELY)

}