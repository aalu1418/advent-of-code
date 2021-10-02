use std::collections::HashMap;

pub fn thirteen(input: Vec<String>) -> (String, String) {
    let input: u64 = input[0].parse().unwrap();

    let init = (1, 1, 0); // x, y, moves
    let mut history = HashMap::new();
    history.insert((init.0, init.1), true);
    let mut stack = Vec::new();
    stack.push(init);

    let mut ans1 = 0;
    let mut ans2 = 0;
    loop {
        if stack.len() == 0 {
            println!("No path found");
            break;
        }

        // get first instance
        let s = stack[0].clone();
        stack.drain(0..1);

        // check end condition
        if s.0 == 31 && s.1 == 39 {
            ans1 = s.2;
            break;
        }

        // check for part 2 condition
        if s.2 == 50 && ans2 == 0 {
            ans2 = history.len();
        }

        // create and append next possible steps
        for i in ["+x", "-x", "+y", "-y"] {
            // skip if at an edge
            if (i == "-x" && s.0 == 0) || (i == "-y" && s.1 == 0) {
                continue;
            }

            // create next step
            let mut temp = s.clone();
            if i.contains("x") {
                if i.contains("-") {
                    temp.0 -= 1;
                } else {
                    temp.0 += 1;
                }
            } else {
                if i.contains("-") {
                    temp.1 -= 1;
                } else {
                    temp.1 += 1;
                }
            }

            // check if it can be added to stack + history
            if !wall(temp.0, temp.1, input) && !history.contains_key(&(temp.0, temp.1)) {
                history.insert((temp.0, temp.1), true);
                temp.2 += 1;
                stack.push(temp);
            }
        }
    }

    (ans1.to_string(), ans2.to_string())
}

fn wall(x: u64, y: u64, b: u64) -> bool {
    let sum_binary = format!("{:b}", x * x + 3 * x + 2 * x * y + y + y * y + b);
    let mut total = 0;
    for v in sum_binary.split("") {
        if v == "1" {
            total += 1;
        }
    }
    total % 2 == 1
}
