use std::thread;
use std::time::Duration;

pub fn main() {
  // 1. create a new thread
  // for i in 1..10 {
  //   thread::spawn(|a| {
  //     println!("thread: number {}!", i);
  //     thread::sleep(Duration::from_millis(100));
  //   });
  // }
  println!("hi from the main thread!");
}
