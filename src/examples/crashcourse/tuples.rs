// Max 12 elements

pub fn main() {
  let person: (&str, &str, i8) = ("Guilherme", "Brasil", 28);

  println!("{} is from {} and is {}", person.0, person.1, person.2);
}
