fn main() {
    let x = 5;

    if x > 10 {
        print!("condition true");
    } else {
        print!("condition false");
    }

    // Throw error
    // let y = 5;
    // if y {
    //     print!("true");
    // }

    // Similar with ternary

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Will give an error
    
    // let number2 = if condition { 6 } else { "other value "};
}
