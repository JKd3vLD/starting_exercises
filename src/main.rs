use std::io;

fn main() {
  println!("\n============== Temperature calculator ==============\n");

  let mut temperature = String::new();
  let mut from = String::new();
  let mut to = String::new();

  let temperature: f32 = loop {
    temperature.clear();
    println!("Enter the temperature in numbers only: ");
    io::stdin()
      .read_line(&mut temperature)
      .expect("Failed to read line");

    match temperature.trim().parse::<f32>() {
      Ok(t) => break t,
      Err(_) => {
        println!("\nInput a valid number to continue!\n");
        continue;
      }
    }
  };

  println!();
  let from: char = loop {
    from.clear();
    println!(
      "Choose the temperature scale you inputted.\n(C - Celsius / F - Fahrenheit / K - Kelvin)\n"
    );
    io::stdin()
      .read_line(&mut from)
      .expect("Failed to read line");

    match from.to_ascii_uppercase().trim() {
      "C" => break 'C',
      "F" => break 'F',
      "K" => break 'K',
      _ => {
        println!("\nInput a valid scale to continue!\n");
        continue;
      }
    };
  };

  println!();
  let to: char = loop {
    to.clear();
    println!(
      "Choose the temperature scale you want to convert to.\n(C - Celsius / F - Fahrenheit / K - Kelvin)\n"
    );
    io::stdin().read_line(&mut to).expect("Failed to read line");

    match to.to_ascii_uppercase().trim() {
      _ if to.to_ascii_lowercase().trim() == format!("{}", from.to_ascii_lowercase()).trim() => {
        println!("\nTry converting to a different scale! ;)\n");
        continue;
      }
      "C" => break 'C',
      "F" => break 'F',
      "K" => break 'K',
      _ => {
        println!("\nInput a valid scale to continue!\n");
        continue;
      }
    };
  };

  let temperature = convert_temperature(temperature, from, to);

  println!("\nThe converted temperature is: {temperature:.2} º{to}");
}

fn convert_temperature(temperature: f32, from: char, to: char) -> f32 {
  match from {
    'C' => match to {
      'F' => temperature * 1.8 + 32.0,
      'K' => temperature + 273.15,
      _ => temperature,
    },
    'F' => match to {
      'C' => (temperature - 32.0) * 5.0 / 9.0,
      'K' => (temperature - 32.0) * 5.0 / 9.0 + 273.15,
      _ => temperature,
    },
    'K' => match to {
      'C' => temperature - 273.15,
      'F' => (temperature - 273.15) * 1.8 + 32.0,
      _ => temperature,
    },
    _ => temperature,
  }
}
