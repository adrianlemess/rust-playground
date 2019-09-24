const MAX_POINTS: u32 = 100_000;

fn main() {
   let x: i8 = 126;

   // Float numbers
   let x = 2.0; // f64

   let y: f32 = 3.0; // f32

   // Operations

   // addition
   let sum = 5 + 10;

   // subtraction
   let difference = 95.5 - 4.3;

   // multiplication
   let product = 4 * 30;

   // division
   let quotient = 56.7 / 32.2;

   // remainder
   let remainder = 43 % 5;

   // Tuple
   let tup: (i32, f64, u8) = (500, 6.4, 1);

   let (x, y, z) = tup;

   println!("the first tupla value is {}", y);
   println!("Print value by the index {}", tup.0);

   // Repeting the same value in array

   let a = [3; 5];

    let first = a[0];
    let second = a[1];

   // Invalid index
   // let a = [1, 2, 3, 4, 5];
   // let index = 10;

   // let element = a[index];

   // println!("The value of element is: {}", element);
}
