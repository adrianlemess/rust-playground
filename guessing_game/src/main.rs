// I/O library. std é a standard library
use std::io;
// Its a enum with the Less Greater and Equal variants
// Compare two values and can reuslt in this three Variants
use rand::Rng;
use std::cmp::Ordering;

// entrypoint application
fn main() {
    // println! is a macro. macro !== function
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");

        // let is used to create a variable
        // mut make variable mutable, by default a variable is imutable
        // String::new() create a new instance of string
        let mut guess = String::new();

        // io::stdin() function handle of standard input
        // .read_line(&mut guess) call read_line method passing the guess argument to me
        // muted by the user input
        // The parameter passed by read_line must be a mutable string
        // The & indicates the argument is a reference (pass only the memory reference and not creating a copy of the object)
        // References by defaul is imutable too thats why we need pass the &mut
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // its for handle the Err value of Result object (Ok and Err)

        // The {} is a placeholder for the guess variable
        println!("You guessed: {}", guess);

        // Rust allow us to reuse the same variable name when we want to change the type value
        // This is call shadowing
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Match is an arm pattern - pattern match and this is how we call
        // By default Rust cannot compare a string with a number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
