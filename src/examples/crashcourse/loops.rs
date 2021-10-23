pub fn main() {
  let mut count = 0;

  // Infinite Loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count >= 10 {
      break;
    }
  }

  println!("-----");

  // For loop (FizzBuzz)
  // for n in 0..101 {
  for n in 0..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }

  println!("-----");

  // While loop (fizzbuzz)
  while count <= 100 {
    if count % 15 == 0 {
      println!("fizzbuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", count);
    }

    count += 1;
  }
}
