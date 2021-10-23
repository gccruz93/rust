pub fn main() {
  println!("{}", raindrops1(35));
  println!("{}", raindrops2(3213212));
  println!("{}", raindrops3(7));
  println!("{}", raindrops4(1));
}

fn raindrops1(n: i32) -> String {
  let mut raindrop = String::new();

  if n % 3 == 0 {
    raindrop.push_str("Pling");
  }

  if n % 5 == 0 {
    raindrop.push_str("Plang");
  }

  if n % 7 == 0 {
    raindrop.push_str("Plong");
  }

  if raindrop.is_empty() {
    raindrop = n.to_string();
  }

  raindrop
}

fn raindrops2(n: i32) -> String {
  let mut raindrop = String::new();

  let mut if_is_factor_push = |factor: i32, sound| {
    if n % factor == 0 {
      raindrop.push_str(sound)
    }
  };

  if_is_factor_push(3, "Pling");
  if_is_factor_push(5, "Plang");
  if_is_factor_push(7, "Plong");

  if raindrop.is_empty() {
    raindrop = n.to_string()
  }

  raindrop
}

fn raindrops3(n: i32) -> String {
  let mut raindrop = String::new();

  let drops = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

  for drop in drops {
    if n % drop.0 == 0 {
      raindrop.push_str(drop.1);
    }
  }

  if raindrop.is_empty() {
    raindrop = n.to_string();
  }

  raindrop
}

fn raindrops4(n: i32) -> String {
  let drops = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

  let raindrop = drops
    .iter()
    .filter(|(factor, _)| n % factor == 0)
    .map(|&(_, sound)| sound)
    .collect::<String>();

  if raindrop.is_empty() {
    return n.to_string();
  }

  raindrop
}
