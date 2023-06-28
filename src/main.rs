extern crate argparse;
use argparse::{ArgumentParser, StoreOption};
use cube_scrambler::generate_scramble;

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
    match generate_scramble(n, cube_type) {
        Ok(scramble) => println!("{}", scramble.join(" ")),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
