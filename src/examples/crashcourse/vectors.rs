// Vectors - Resizable arrays

use std::mem;

pub fn main() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(6);
  numbers.push(7);

  // Pop off last value
  numbers.pop();

  // Print vector with debug trait {:?}
  println!("{:?}", numbers);

  // Get single value
  println!("Single value: {}", numbers[0]);

  // Get vector length
  println!("Vector length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  println!("-----");

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  println!("-----");

  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers Vec: {:?}", numbers);

  println!("-----");

  // Loop with filter
  for x in numbers.iter().filter(|&n| *n > 5) {
    println!("Number greater then 5: {}", x);
  }
}
