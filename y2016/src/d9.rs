pub fn nine(input: Vec<String>) -> (String, String) {
    let input = &input[0];
    let input: Vec<char> = input.chars().collect();

    let count = sub_fn(&input, false);
    let count2 = sub_fn(&input, true);

    (count.to_string(), count2.to_string())
}

fn sub_fn(input: &Vec<char>, re: bool) -> usize {
    let mut i = 0;
    let mut check = false;
    let mut params: Vec<char> = Vec::new();
    let mut count: usize = 0;
    while i < input.len() {
        if check && input[i] == ')' {
            check = false;
            let s: String = params.iter().collect();
            let s: Vec<&str> = s.split("x").collect();
            let a: usize = s[0].parse().unwrap();
            let b: usize = s[1].parse().unwrap();

            if re {
                let mut sub_str: Vec<char> = Vec::new();
                for ii in 0..a {
                    sub_str.push(input[i + 1 + ii]);
                }
                count += sub_fn(&sub_str, re) * b;
            } else {
                count += a * b;
            }

            i += a + 1;
            params = Vec::new(); // reset

            continue;
        }

        if check {
            params.push(input[i]);
        }

        if input[i] == '(' {
            check = true;
        }

        if !check {
            count += 1;
        }

        i += 1;
    }

    return count;
}
