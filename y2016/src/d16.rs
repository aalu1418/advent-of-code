pub fn sixteen(input: Vec<String>) -> (String, String) {
    let base = input[0].clone();
    let max = 272;

    let base2 = input[0].clone();
    let max2 = 35651584;

    (checksum(fill(base, max)), checksum(fill(base2, max)))
}

fn fill(base: String, length: usize) -> String {
    if base.len() >= length {
        return base[0..length].to_string();
    }

    let a = base.clone();
    let b: String = base
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => 'E',
        })
        .rev()
        .collect();

    return fill(a + "0" + &b, length);
}

fn checksum(base: String) -> String {
    if base.len() % 2 == 1 {
        return base;
    }

    let out: String = base
        .char_indices()
        .map(|(i, c)| {
            if i % 2 == 0 {
                if c == base.chars().nth(i + 1).unwrap() {
                    "1"
                } else {
                    "0"
                }
            } else {
                ""
            }
        })
        .collect();
    return checksum(out);
}
