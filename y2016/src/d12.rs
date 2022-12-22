use std::collections::HashMap;

// naive solution (second answer takes a while to run)
pub fn twelve(input: Vec<String>) -> (String, String) {
    let input = &input[0];

    let b0 = run(&input, 0, 0);
    let b1 = run(&input, 0, 1);
    (b0, b1)
}

pub fn run(input: &String, a_val: isize, c_val: isize) -> String {
    // create buckets
    let mut buckets = HashMap::new();
    buckets.insert("a", a_val);
    buckets.insert("b", 0 as isize);
    buckets.insert("c", c_val);
    buckets.insert("d", 0 as isize);

    let mut input_list: Vec<Vec<&str>> =
        input.split("\n").map(|x| x.split(" ").collect()).collect();
    let mut i: isize = 0;
    while i < input_list.len() as isize {
        let l = input_list[i as usize].clone();
        match l[0] {
            "cpy" => {
                let mut chars = l[1].chars();
                let mut in_two_chars = l[2].chars();

                // skip invalid instruction
                if !in_two_chars.next().unwrap().is_alphabetic() {
                    i += 1;
                    continue;
                }

                if chars.next().unwrap().is_alphabetic() {
                    buckets.insert(l[2], buckets[l[1]]);
                } else {
                    buckets.insert(l[2], l[1].parse().unwrap());
                }
            }
            "inc" => {
                *buckets.get_mut(l[1]).unwrap() += 1;
            }
            "dec" => {
                *buckets.get_mut(l[1]).unwrap() -= 1;
            }
            "jnz" => {
                let mut chars = l[1].chars();
                let mut in_two_chars = l[2].chars();
                let val;
                let step;
                if chars.next().unwrap().is_alphabetic() {
                    val = buckets[l[1]];
                } else {
                    val = l[1].parse().unwrap();
                }
                if in_two_chars.next().unwrap().is_alphabetic() {
                    step = buckets[l[2]];
                } else {
                    step = l[2].parse().unwrap();
                }

                if val != 0 {
                    i += step;
                    continue;
                }
            }
            "tgl" => {
                let ind: isize = i + buckets[l[1]] as isize;

                if ind < 0 || ind > (input_list.len() - 1) as isize {
                    i += 1;
                    continue; // skip if invalid index
                }

                let mut cmd: Vec<&str> = input_list[ind as usize].clone();
                if cmd.len() == 2 {
                    cmd[0] = match cmd[0] {
                        "inc" => "dec",
                        _ => "inc",
                    }
                } else if cmd.len() == 3 {
                    cmd[0] = match cmd[0] {
                        "jnz" => "cpy",
                        _ => "jnz",
                    }
                }

                input_list[ind as usize] = cmd;
            }
            "out" => {
                let val;
                let mut chars = l[1].chars();
                if chars.next().unwrap().is_alphabetic() {
                    val = buckets[l[1]];
                } else {
                    val = l[1].parse().unwrap();
                }
                println!("{}", val);
            }
            _ => {
                println!("'{}' not found", l[0])
            }
        }
        i += 1;
    }
    println!(
        "BRUTE FORCE: {:?}",
        [buckets["a"], buckets["b"], buckets["c"], buckets["d"]]
    );

    return buckets["a"].to_string();
}
