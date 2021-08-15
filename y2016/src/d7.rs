use regex::Regex;

pub fn seven(input: Vec<String>) -> (String, String) {
    let mut count = 0;
    let mut count2 = 0;
    let re = Regex::new(r"\[\w*\]").unwrap();

    for l in input[0].split("\n") {
        let l_vec: Vec<char> = l.chars().collect();
        if check(&l_vec) {
            let mut fail = false;
            for i in re.captures_iter(&l) {
                if check(&i[0].chars().collect()) {
                    fail = true;
                    break;
                }
            }

            if !fail {
                count += 1;
            }
        }

        if check2(
            l.to_string(),
            re.find_iter(l)
                .filter_map(|d| d.as_str().parse().ok())
                .collect(),
        ) {
            count2 += 1
        }
    }

    (count.to_string(), count2.to_string())
}

fn check(input: &Vec<char>) -> bool {
    for i in 0..input.len() {
        if i + 3 >= input.len() {
            break;
        }
        if input[i] == input[i + 3] && input[i + 1] == input[i + 2] && input[i] != input[i + 1] {
            return true;
        }
    }
    return false;
}

fn check2(mut input: String, check: Vec<String>) -> bool {
    for c in check.iter() {
        // remove bracketed components from string
        input = input.replace(c, " ");
    }
    let input: Vec<char> = input.chars().collect();

    // find ABA match
    for i in 0..input.len() {
        if i + 2 >= input.len() {
            break;
        }
        if input[i] == input[i + 2] && input[i] != input[i + 1] {
            let substr: String = [input[i + 1], input[i], input[i + 1]].iter().collect();
            for c in check.iter() {
                // check BAB match in brackets
                if c.contains(&substr) {
                    return true;
                }
            }
        }
    }
    return false;
}
