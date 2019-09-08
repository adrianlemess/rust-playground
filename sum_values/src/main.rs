use std::io;

fn main() {
    println!("Sum values!");

    let mut input_a = String::new();
    let mut input_b = String::new();

    println!("Enter the first value: ");
    io::stdin().read_line(&mut input_a)
        .expect("failed to read line");


    println!("Enter the second value: ");
    io::stdin().read_line(&mut input_b)
        .expect("failed to read line");

    let number_a: u32 = input_a.trim().parse().expect("Not a number!");
    let number_b: u32 = input_b.trim().parse().expect("Not a number!");

    print!("The result of {} + {} is {} \n", number_a, number_b, (number_a + number_b));
}
