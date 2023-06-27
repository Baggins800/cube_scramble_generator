extern crate argparse;
use rand::Rng;
use argparse::{ArgumentParser, StoreOption};

fn generate_scramble(arg: Option<u32>, cube_type: Option<String>) -> Vec<String>{
  let range = 1 ..= match arg {
    Some(v) => v,
    None => match cube_type.as_deref() {
        Some("2x2") => 9,
        Some("3x3") => 20,
        Some("4x4") => 46,
        Some("5x5") => 60,
        Some(&_) => 20,
        None => 20
    },
  };
  let notationrange = match cube_type.as_deref() {
        Some("2x2") => 3,
        Some("3x3") => 6,
        Some("4x4") => 9,
        Some("5x5") => 12,
        Some(&_) => 6,
        None => 6
    };

  let notations = vec!["R", "F", "U", // 2x2
    "L", "B", "D", // 3x3
    "Rw", "Fw", "Uw", // 4x4
    "Bw", "Lw", "Dw" // 5x5
  ];
  let extras = vec!["", "'", "2"];
  let mut rand_generator = rand::thread_rng();
  let mut last_notation: Option<u32> = None;
  let selected_notations: Vec<(u32, u32)> = range
    .map(|_| {
        let mut selected_notation = rand_generator.gen_range(0..notationrange as u32);
        let selected_extra = rand_generator.gen_range(0..extras.len() as u32);
        while Some(selected_notation) == last_notation {
          selected_notation = rand_generator.gen_range(0..notationrange as u32);
        }
        last_notation = Some(selected_notation);
        (selected_notation, selected_extra)
    })
    .collect();
  let result: Vec<String> = selected_notations
      .into_iter()
      .map(|(x, y)| {
        format!("{}{}", notations[x as usize], extras[y as usize])
      })
      .collect();
  return result;
}

fn main() {
  let mut n: Option<u32> = None;
  let mut cube_type: Option<String> = None;
  {
    let mut parser = ArgumentParser::new();
    parser.set_description("2x2, 3x3, 4x4 and 5x5 scramble generator");
    parser.refer(&mut n)
        .add_option(&["-c", "--count"], StoreOption, "Set the number of notations.");
    parser.refer(&mut cube_type)
        .add_option(&["-t", "--type"], StoreOption, "Set the cube type, 2x2, 3x3, 4x4 or 5x5");
    parser.parse_args_or_exit();
  }
  println!("{}", generate_scramble(n, cube_type).join(" "));
}
