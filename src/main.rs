fn main() {
    println!("Hello, world!");
}

// fn to convert
// fn to ask for number
// lets start with a test

fn convert_c_to_f(n: i32) -> i32 {
    let x = n;
    0
}

fn convert_f_to_c(n: i32) -> i32 {
    let x = n;
    0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_c_to_f() {
      // It should correctly convert a C temperature to F.
      assert_eq!(convert_c_to_f(-40), -40);
      assert_eq!(convert_c_to_f(0), -18);
      assert_eq!(convert_c_to_f(32), 0);
  }

  #[test]
  fn test_convert_f_to_c() {
      // It should correctly convert a C temperature to F.
      assert_eq!(convert_c_to_f(-40), -40);
      assert_eq!(convert_c_to_f(0), 32);
      assert_eq!(convert_c_to_f(40), 104);
  }
}
