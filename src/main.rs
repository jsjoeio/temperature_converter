fn main() {
    println!("Hello, world!");
}

fn convert_c_to_f(n: f64) -> f64 {
    (n * 1.8) + 32.0
}

fn convert_f_to_c(n: f64) -> f64 {
    (n - 32.0) / 0.5556
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_c_to_f() {
      // It should correctly convert a C temperature to F.
      assert_eq!(convert_c_to_f(-40.0), -40.0);
      assert_eq!(convert_c_to_f(0.0), 32.0);
      assert_eq!(convert_c_to_f(32.0), 89.6);
  }

  #[test]
  fn test_convert_f_to_c() {
      // It should correctly convert a C temperature to F.
      assert_eq!(convert_c_to_f(-40.0), -40.0);
      assert_eq!(convert_c_to_f(0.0), 32.0);
      assert_eq!(convert_c_to_f(40.0), 104.0);
  }
}
