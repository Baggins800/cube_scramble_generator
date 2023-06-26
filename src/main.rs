extern crate argparse;
use rand::Rng;
use argparse::{ArgumentParser, StoreOption};

fn generate_scramble(arg: Option<u32>) -> Vec<String>{
  let range = 1 ..= match arg {
    Some(v) => v,
    None => 20,
  };
  let notations = vec!["R", "L", "B", "F", "U", "D"];
  let extras = vec!["", "'", "2"];
  let mut rand_generator = rand::thread_rng();
  let mut last_notation: Option<u32> = None;
  let selected_notations: Vec<u32> = range.clone()
    .map(|_| {
        let mut selected = rand_generator.gen_range(0..notations.len() as u32);
        while Some(selected) == last_notation {
          selected = rand_generator.gen_range(0..notations.len() as u32);
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
  {
    let mut parser = ArgumentParser::new();
    parser.set_description("3x3 scramble generator");
    parser.refer(&mut n)
        .add_option(&["-c", "--count"], StoreOption, "Set the number of notations.");
    parser.parse_args_or_exit();
  }
  println!("{}", generate_scramble(n).join(" "));
}
