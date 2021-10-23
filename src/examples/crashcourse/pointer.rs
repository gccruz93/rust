// Reference Pointers - Point to a resource in memoty

pub fn main() {
  // Primitive Array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("{:?}", (arr1, arr2));
  println!("-----");

  // Vectors
  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1; // reference

  println!("{:?}", (&vec1, vec2));
}
