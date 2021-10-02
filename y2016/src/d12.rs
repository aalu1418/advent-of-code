use std::collections::HashMap;

// naive solution (second answer takes a while to run)
pub fn twelve(input: Vec<String>) -> (String, String) {
    let input = &input[0];

    let b0 = run(&input, 0);
    let b1 = run(&input, 1);
    (b0, b1)
}

fn run(input: &String, c_val: usize) -> String {
    // create buckets
    let mut buckets = HashMap::new();
    buckets.insert("a", 0);
    buckets.insert("b", 0);
    buckets.insert("c", c_val);
    buckets.insert("d", 0);

    let input_list: Vec<&str> = input.split("\n").collect();
    let mut i: isize = 0;
    while i < input_list.len() as isize {
        let l = input_list[i as usize];
        let l: Vec<&str> = l.split(" ").collect();

        match l[0] {
            "cpy" => {
                let mut chars = l[1].chars();
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
                let val;
                if chars.next().unwrap().is_alphabetic() {
                    val = buckets[l[1]];
                } else {
                    val = l[1].parse().unwrap();
                }
                if val != 0 {
                    i += l[2].parse::<isize>().unwrap();
                    continue;
                }
            }
            _ => {
                println!("'{}' not found", l[0])
            }
        }
        i += 1;
    }

    return buckets["a"].to_string();
}
