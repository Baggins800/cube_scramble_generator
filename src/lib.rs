use rand::Rng;

fn cube_settings(cube_type: Option<String>) 
    -> Result<(u32, u32, Vec<&'static str>), &'static str> {
        let (notation_range, default_count) = 
            match cube_type.as_deref() {
                Some("2x2") => (3, 9),
                Some("3x3") => (6, 20),
                Some("4x4") => (9, 46),
                Some("5x5") => (12, 60),
                Some("6x6") => (15, 80),
                Some("7x7") => (18, 100),
                Some(&_) => {
                    return Err("Invalid cube type provided.\
                               Valid types are: 2x2, 3x3, 4x4, 5x5, 6x6, 7x7.")
                }
                None => (6, 20)
            };
        Ok((notation_range, default_count,
            vec![
            "R", "F", "U",       // 2x2
            "L", "B", "D",       // 3x3
            "Rw", "Fw", "Uw",    // 4x4
            "Bw", "Lw", "Dw",    // 5x5
            "3Rw", "3Fw", "3Uw", // 6x6
            "3Bw", "3Lw", "3Dw"  // 7x7
            ]))
}

pub fn generate_regular_scramble(arg: Option<u32>, cube_type: Option<String>) 
    -> Result<Vec<String>, &'static str> {
        let (notation_range, count, notations) = 
            match cube_settings(cube_type) {
                Ok(x) => x,
                Err(x) => return Err(x)
            };
        let range = 1 ..= match arg {
            Some(v) => v,
            None =>count 
        };

        let extras = vec!["", "'", "2"];
        let mut rand_generator = rand::thread_rng();
        let mut last_notation: Option<u32> = None;
        let selected_notations: Vec<(u32, u32)> = range
            .map(|_| {
                let mut selected_notation = 
                    rand_generator.gen_range(0..notation_range as u32);
                let selected_extra = 
                    rand_generator.gen_range(0..extras.len() as u32);
                while Some(selected_notation) == last_notation {
                    selected_notation = 
                        rand_generator.gen_range(0..notation_range as u32);
                }
                last_notation = Some(selected_notation);
                (selected_notation, selected_extra)
            })
        .collect();
        Ok(selected_notations
           .into_iter()
           .map(|(x, y)| {
               format!("{}{}", notations[x as usize], extras[y as usize])
           })
           .collect())
}

pub fn generate_scramble(arg: Option<u32>, cube_type: Option<String>)
    -> Result<Vec<String>, &'static str> {
  match cube_type.as_deref() {
    None |
    Some("2x2") |
    Some("3x3") |
    Some("4x4") |
    Some("5x5") |
    Some("6x6") |
    Some("7x7") => generate_regular_scramble(arg, cube_type),
    Some(_) => Err("Invalid Cube")
  }
}
