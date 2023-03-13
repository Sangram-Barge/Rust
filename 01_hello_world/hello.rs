fn main() {
  test();
  simple_expression();
}

fn test() -> () {
  println!("this is from function")
}

fn simple_expression() -> () {
  let x = 5 * 20;
  println!("what is value of x {}", x)
}