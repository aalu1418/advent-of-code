use regex::Regex;
use std::collections::HashMap;

pub fn four(input: Vec<String>) -> (String, String) {
    let re = Regex::new(r"([a-z\-]+)\-(\d+)\[([a-z]+)\]").unwrap();
    let mut sum: u32 = 0;
    let mut id: u32 = 0;

    for l in re.captures_iter(&input[0]) {
        let mut chars: Vec<char> = l[1].chars().collect();
        let mut original = chars.clone();
        chars.sort(); // sort alphabetically
        let sorted = chars.clone();
        chars.dedup(); // remove duplicates
        let mut count = HashMap::new();
        // count instances
        for c in sorted {
            match c {
                '-' => count.insert(c, 0),
                _ => {
                    if !count.contains_key(&c) {
                        count.insert(c, 0);
                    }
                    count.insert(c, count[&c] + 1)
                }
            };
        }
        chars.sort_by(|a, b| count[b].cmp(&count[a])); // sort by count
        let checksum: String = chars[0..5].into_iter().collect();

        if checksum == &l[3] {
            let step = &l[2].parse().unwrap();
            sum += step;

            for i in 0..original.len() {
                if original[i] == '-' {
                    original[i] = ' ';
                    continue;
                }
                original[i] = ('a' as u8
                    + (((original[i] as u8 - 'a' as u8) as u32 + step) % 26) as u8)
                    as char
            }

            let res: String = original.into_iter().collect();
            if res.contains("north") {
                id += step;
            }
        }
    }

    (sum.to_string(), id.to_string())
}
