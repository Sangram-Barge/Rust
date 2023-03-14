fn main() -> () {
  // println!("this is simple {}", "text");

  // println!("{1}, {0}, {0}", "sangram", "anushree");

  // println!("{web_dev}, {data_dev}", web_dev="sangram is FSD", data_dev="anu is BDD");

  // println!("number formatting");
  // println!("decimal {}", 23876);
  // println!("binary {:b}", 23876);
  // println!("octal {:o}", 23876);
  // println!("hex {:x}", 23876);
  // println!("big hex {:X}", 23876);

  // println!("________________________________");

  // println!("{number:>5}", number=1);
  // println!("{number:0<5}", number=1);

  let number: f64 = 1.0;
  let width: usize = 5;
  println!("{number:>width$}");

}