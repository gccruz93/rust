// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct person
  fn new(first_name: &str, last_name: &str) -> Person {
    Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
    }
  }

  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", &self.first_name, &self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last_name: &str) {
    self.last_name = last_name.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn main() {
  let white = Color {
    red: 255,
    green: 255,
    blue: 255,
  };

  println!("{}, {}, {}", white.red, white.green, white.blue);

  println!("-----");

  let green = TupleColor(0, 255, 0);

  println!("{}, {}, {}", green.0, green.1, green.2);

  println!("-----");

  let mut p = Person::new("Guilherme", "Cruz");
  println!("Person {}", p.full_name());
  p.set_last_name("Alves");
  println!("Person {}", p.full_name());
  println!("Person {:?}", p.to_tuple());

  println!("-----");
}
