extern crate argparse;
use rand::Rng;
use argparse::{ArgumentParser, StoreOption};

fn generate_scramble(arg: Option<u32>, cube_type: Option<String>) -> Vec<String>{
  let range = 1 ..= match arg {
    Some(v) => v,
    None => match cube_type.as_deref() {
        Some("2x2") => 9,
        Some("3x3") => 20,
        Some(&_) => 20,
        None => 20
    },
  };
  let notationrange = match cube_type.as_deref() {
        Some("2x2") => 3,
        Some("3x3") => 6,
        Some(&_) => 6,
        None => 6
    };

  let notations = vec!["R", "F", "U", "L", "B", "D"];
  let extras = vec!["", "'", "2"];
  let mut rand_generator = rand::thread_rng();
  let mut last_notation: Option<u32> = None;
  let selected_notations: Vec<u32> = range.clone()
    .map(|_| {
        let mut selected = rand_generator.gen_range(0..notationrange as u32);
        while Some(selected) == last_notation {
          selected = rand_generator.gen_range(0..notationrange as u32);
        }
        last_notation = Some(selected);
        selected
    })
    .collect();

  let mut last_extra: Option<u32> =  None;
  let selected_extras: Vec<u32> = range.clone()
    .map(|_| {
      let mut selected = rand_generator.gen_range(0..extras.len() as u32);
      while Some(selected) == last_extra {
        selected = rand_generator.gen_range(0..extras.len() as u32);
      }
      last_extra = Some(selected);
      selected
    }).collect();
  let result: Vec<String> = range
    .map(|x| {
      format!("{}{}", notations[selected_notations[(x - 1) as usize] as usize].to_string(),
      extras[selected_extras[(x - 1) as usize] as usize].to_string())
    }).collect();
  return result;
}

fn main() {
  let mut n: Option<u32> = None;
  let mut cube_type: Option<String> = None;
  {
    let mut parser = ArgumentParser::new();
    parser.set_description("2x2 and 3x3 scramble generator");
    parser.refer(&mut n)
        .add_option(&["-c", "--count"], StoreOption, "Set the number of notations.");
    parser.refer(&mut cube_type)
        .add_option(&["-t", "--type"], StoreOption, "Set the cube type, 2x2 or 3x3");
    parser.parse_args_or_exit();
  }
  println!("{}", generate_scramble(n, cube_type).join(" "));
}
