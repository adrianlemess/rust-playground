fn main() {
    // Infinite loop
    // loop {
    //
    // }

    // Breaking loop
    // let mut counter = 0;

    // loop {
    //     println!("Ao finito e não além {}", counter);
    //     counter = counter + 1;

    //     if counter > 10 {
    //         break;
    //     }
    // }

    // Returning value from looping

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     println!("Counter: {}", counter);
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    // Condition loop

    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // Print a collection with while

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // Print a collection with a more efficient and perfomatic way

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("element is {}", element);
    // }

    // Counter 1 to 3

    // for number in (1..4) {
    //     println!("Number is: {}", number);
    // }

    // Reverse counter 3 ... 1

    for number in (1..4).rev() {
        println!("Number is {}", number);
    }
}
