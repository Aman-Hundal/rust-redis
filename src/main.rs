use mini-redis::{client, Result};

// Rust transforms the async fn at compile time into a routine that operates asynchronously. Any calls to .await within the async fn yield control back to the thread. The thread may do other work while the operation processes in the background.
// Although other languages implement async/await too, Rust takes a unique approach. Primarily, Rust's async operations are lazy. This results in different runtime semantics than other languages.
// calling these functions does not result in the function body executing. Instead, calling an async fn returns a value representing the operation. To actually run the operation, you should use the .await operator on the return value.

// However, asynchronous functions must be executed by a runtime. The runtime contains the asynchronous task scheduler, provides evented I/O, timers, etc. The runtime does not automatically start, so the main function needs to start it.
//The #[tokio::main] function is a macro. It transforms the async fn main() into a synchronous fn main() that initializes a runtime instance and executes the async main function

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;
    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;
    // Get key "hello"
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    Ok(())
}
