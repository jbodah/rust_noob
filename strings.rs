fn main() {
  println!("{} days", 31);
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  println!("{subject} {verb} {object}",
           object = "the lazy dog",
           subject = "the quick brown fox",
           verb = "jumps over");

  // Right-align with specified width
  println!("{number:>width$}", number = 1, width = 6);

  // Pad with 0's
  println!("{number:>0width$}", number = 1, width = 6);

  // Create struct which contains an i32
  //struct Structure(i32);

  // breaks
  //println!("This struct `{}` won't print...", Structure(3));

  // std::fmt contains traits which govern display of text
  // * fmt::Debug uses {:?} marker for debugging
  // * fmt::Display uses {} for user friendly

  // 3.143
  println!("Pi is roughly {:.*}", 3, 22.0/7.0);
}
