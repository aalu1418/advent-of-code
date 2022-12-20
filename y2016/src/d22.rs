use std::num::ParseIntError;

pub fn twentytwo(input: Vec<String>) -> (String, String) {
    let mut disks: Vec<(&str, usize, usize)> = input[0]
        .lines()
        .filter_map(|x| {
            let v: Vec<&str> = x.split_whitespace().collect();
            if v.len() < 5 {
                return None;
            }

            let used = match parse(v[2]) {
                Ok(v) => v,
                Err(_) => {
                    return None;
                }
            };
            let avail = match parse(v[3]) {
                Ok(v) => v,
                Err(_) => {
                    return None;
                }
            };

            Some((v[0], used, avail))
        })
        .collect();
    disks.sort_by(|a, b| a.2.cmp(&b.2));

    (valid_pairs(&disks).to_string(), "".to_string())
}

fn parse(s: &str) -> Result<usize, ParseIntError> {
    let mut v = s.to_string();
    v.pop();
    v.parse()
}

fn valid_pairs(disks: &Vec<(&str, usize, usize)>) -> usize {
    let mut count = 0;

    for d in disks {
        count += disks
            .iter()
            .filter(|x| x.2 >= d.1 && x.0 != d.0 && d.1 != 0)
            .count();
    }

    return count;
}
