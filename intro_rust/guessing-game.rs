user std::io;

fn main() {
    println!("Please enter a number");
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess);

    println!("You guessed: {}", guess)




}
