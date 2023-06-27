use rand::Rng;

pub fn generate_scramble(arg: Option<u32>, cube_type: Option<String>) -> Vec<String>{
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
    selected_notations
        .into_iter()
        .map(|(x, y)| {
            format!("{}{}", notations[x as usize], extras[y as usize])
        })
    .collect()
}
