pub fn sixteen(input: Vec<String>) -> (String, String) {
    let base: Vec<char> = input[0].clone().chars().collect();
    let max = 272;

    let base2 = base.clone();
    let max2 = 35651584;

    (checksum(fill(base, max)), checksum(fill(base2, max2)))
}

fn fill(base: Vec<char>, length: usize) -> Vec<char> {
    if base.len() >= length {
        return base[0..length].to_vec();
    }

    let mut out = base.clone();
    let mut b: Vec<char> = base
        .iter()
        .map(|&c| match c {
            '1' => '0',
            '0' => '1',
            _ => 'E',
        })
        .rev()
        .collect();

    out.push('0');
    out.append(&mut b);

    return fill(out, length);
}

fn checksum(base: Vec<char>) -> String {
    if base.len() % 2 == 1 {
        return base.iter().collect();
    }

    let out: Vec<char> = base
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(i, c)| if *c == base[i + 1] { '1' } else { '0' })
        .collect();
    return checksum(out);
}
