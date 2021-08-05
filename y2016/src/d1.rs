use std::collections::HashMap;

pub fn one(input: Vec<String>) -> (String, String) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: i32 = 0; // N - 0, E - 1, S - 2, W - 3
    let mut history = HashMap::new();
    history.insert((0, 0), true);
    let mut repeat_dist: i32 = 0;

    for line in input[0].split(", ") {
        let (lr, step) = line.split_at(1);
        let step: i32 = step.parse().unwrap();

        let lr_step = if lr == "L" { -1 } else { 1 };
        d = if (d + lr_step) == -1 {
            3
        } else {
            (d + lr_step) % 4
        };

        let mul = if d >= 2 { -1 } else { 1 };

        for _ in 0..step {
            if d % 2 == 0 {
                y += mul;
            } else {
                x += mul;
            }

            let check = history.insert((x, y), true);
            if check != None && repeat_dist == 0 {
                repeat_dist = x.abs() + y.abs();
            }
        }
    }

    ((x.abs() + y.abs()).to_string(), repeat_dist.to_string())
}
