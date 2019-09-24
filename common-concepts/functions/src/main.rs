fn main() {
    println!("Hello, world!");

    another_function();
    func_with_parameter(5);
    func_with_multi_parameters(5, 10);

    // Assign an expression to a block of code

    let x = 5;

    let y = {
        let x = 3;

        // Expressions don't have semi-colon(;), with semi-colom became an statement without return
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of plus 1 is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn func_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn func_with_multi_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
