fn main () {
    // Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. 
    
    // STack = The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. 
    // Heap = Data with an unknown size at compile time or a size that might change must be stored on the heap instead. The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

    // Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses

    //Ownership Rules - 1. Each value in Rust has an owner 2. A value can only have one owner at a time 3. When the owner goes out of scope, the value will be dropped.

    // Variable Scope
       {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward. s is equal to a string litteral (DIFFERENT FROM STRING TYPE). String litterlas are immutable while STRINGS are mutable    

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // STring types in rust are a complex data type and are stored in the heap (along with other compelx data structure you will make yourself). Basic data types we have used so far are less complex and of fixed value and therefore on the stack

    // Strings and ownership
    // creating a string from a string litteral
    let s2 = String::from("hello");

    // Strings vs String Litteral key descrapancy is how they are managed in memory - STRINGS go into the heap while string litterals go into the stack

    //when using data types like string we need to in general have two things happen
    // 1. The memory must be requested from the memory allocator at runtime. (we do this when we declare a variable -> universal in programming lanagues)
    // 2.We need a way of returning this memory to the allocator when we’re done with our String. (this varies from lanaguage to language -> either done expliclty by programmer or by a garbage collector)
        // Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free

    // Rust appproach = the memory is automatically returned once the variable that owns it goes out of scope.

        {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        }                                  // this scope is now over, and s is no longer valid

    // When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket

    // Variables and Data Interacting with Move

}