// Enums are types which have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  // Perform action depending on info
  match m {
    Movement::Up => println!("Up"),
    Movement::Down => println!("Down"),
    Movement::Left => println!("Left"),
    Movement::Right => println!("Right"),
  }
}

pub fn main() {
  let avatar1 = Movement::Up;
  let avatar2 = Movement::Down;
  let avatar3 = Movement::Left;
  let avatar4 = Movement::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
  // println!("-----");
}
