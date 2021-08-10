use regex::Regex;

pub fn seven(input: Vec<String>) -> (String, String) {
    let mut count = 0;
    let re1 = Regex::new(r"\[(\w+)\]").unwrap();
    let re2 = Regex::new(r"(\w+)\[?\w+\]?").unwrap();

    'outer: for l in input[0].split("\n") {
        for i in re1.captures_iter(&l) {
            if check(i[1].to_string()) {
                continue 'outer;
            }
        }

        for i in re2.captures_iter(&l) {
            if check(i[1].to_string()) {
                count += 1;
                continue 'outer;
            }
        }
    }

    (count.to_string(), "".to_string())
}

fn check(input: String) -> bool {
    let input: Vec<char> = input.chars().collect();
    for i in 0..input.len() - 4 {
        if input[i] == input[i + 3] && input[i + 1] == input[i + 2] && input[i] != input[i + 1] {
            return true;
        }
    }
    return false;
}
