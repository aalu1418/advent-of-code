use std::collections::HashMap;

pub fn one(input: Vec<String>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut d = 0; // N - 0, E - 1, S - 2, W - 3

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

        if d % 2 == 0 {
            y += mul * step;
        } else {
            x += mul * step;
        }
    }

    (x.abs() + y.abs(), 0)
}
