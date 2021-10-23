pub fn main() {
  greeting("Hello", "Guilherme");

  println!("-----");

  // Bind function values to variables
  let get_sum = add(5, 6);
  println!("Sum: {}", get_sum);

  println!("-----");

  // Closure with scope variables
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure Sum: {}", add_nums(3, 4));

  println!("-----");

  //
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
