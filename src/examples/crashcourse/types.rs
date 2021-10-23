/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: (bool)
Characters: (char)
Tuples
Arrays
*/

pub fn main() {
  // Default is -> i32
  let _x = 1;

  // Default is -> f64
  let _y = 2.5;

  // Add explicit type
  let _z: i64 = 45454545;

  // Find max size
  println!("Max i8: {}", std::i8::MAX);
  println!("Max i16: {}", std::i16::MAX);
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  println!("-----");

  // Boolean
  let is_active: bool = true;

  // Character
  let ch = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (_x, _y, _z, is_active, ch, face));

  println!("-----");

  // Get boolean from expression
  let is_greater = 10 > 5;

  println!("{:?}", is_greater);
}
