use std::io;

fn main() {
    println!("\n༼ つ ◕_◕ ༽つ The temperature converter! ☁︎");

    // Use it to store the either C or F
    let mut unit_type = String::new();

    // Use to store the number to convert
    let mut num_to_convert = String::new();

    // Ask if user wants to convert C to F, or F to C
    loop {
        println!("\nPlease enter the unit you're converting from: C or F");


        io::stdin().read_line(&mut unit_type)
            .expect("Failed to read line");

        // Trim to remove the line break added when the user hits "enter"
        match unit_type.trim() {
            "C" => println!("\nConverting from C to F"),
            "F" => println!("\nConverting from F to C"),
            _ => {
                println!("\nOops...that doesn't look C or F");
                continue;
            },
        };

        break;
    }

    loop {
        println!("\nPlease enter the number you want to convert:");

        io::stdin().read_line(&mut num_to_convert)
            .expect("Failed to read line");

        let num_to_convert: u32 = match num_to_convert.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let converted = match unit_type.trim() {
            "C" => convert_c_to_f(num_to_convert as f64),
            "F" => convert_f_to_c(num_to_convert as f64),
            _ => -1.0, // TODO fix this
        };

        let output_unit = match unit_type.trim() {
            "C" => "F",
            "F" => "C",
            _ => "Unknown type"
        };

        println!("\n{}°{} is equal to {}°{}", num_to_convert, unit_type.trim(), converted.round(), output_unit);
        break;
    }
}

fn convert_c_to_f(n: f64) -> f64 {
    ((n * 1.8) + 32.0).round()
}

fn convert_f_to_c(n: f64) -> f64 {
    ((n - 32.0) * 0.5556).round()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_c_to_f() {
      // It should correctly convert a C temperature to F.
      assert_eq!(convert_c_to_f(-40.0), -40.0);
      assert_eq!(convert_c_to_f(0.0), 32.0);
      assert_eq!(convert_c_to_f(32.0), 90.0);
      assert_eq!(convert_c_to_f(21.0), 70.0);

  }

  #[test]
  fn test_convert_f_to_c() {
      // It should correctly convert a C temperature to F.
      assert_eq!(convert_f_to_c(39.0), 4.0);
      assert_eq!(convert_f_to_c(-40.0), -40.0);
      assert_eq!(convert_f_to_c(100.0), 38.0);
  }
}
