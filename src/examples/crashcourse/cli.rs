use std::env;

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let status = "100%";
  // println!("Args: {:?}", args);
  // println!("Command: {}", command);

  if command == "hello" {
    println!("Hello!");
  } else {
    println!("Status: {}", status);
  }
  // println!("-----");
}
