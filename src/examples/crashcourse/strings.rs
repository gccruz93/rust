// Primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated  data structure - Use when you need to motify or own string data

pub fn main() {
  let mut hello = String::from("Hello");
  let hello2;

  hello2 = "Hello2";

  println!("{} and {}", hello, hello2);

  // Get length
  println!("-----");

  println!("Length of hello: {}", hello.len());

  // Push
  println!("-----");
  hello.push(' ');
  hello.push('W');
  hello.push_str("orld!");
  println!("{}", hello);

  // Capacity in bytes
  println!("-----");
  println!("Capacity: {}", hello.capacity());
  println!("Is Empty: {}", hello.is_empty());

  // Contains
  println!("-----");
  println!("Contains 'World': {}", hello.contains("World"));

  // Replace
  println!("-----");
  println!("Replace: {}", hello.replace("World", "There"));

  // Loop through string by whitespace
  println!("-----");
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  println!("-----");
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  println!("{}", s);

  // Assertion testing
  assert_eq!(2, s.len());
}
