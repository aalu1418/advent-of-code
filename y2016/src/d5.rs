use md5;

pub fn five(input: Vec<String>) -> (String, String) {
    let id = &input[0];
    let mut hashes = Vec::with_capacity(8);
    let mut hash2: [char; 8] = ['Z', 'Z', 'Z', 'Z', 'Z', 'Z', 'Z', 'Z'];

    let mut i = 0;
    while !(hashes.len() == 8 && check(hash2) == 8) {
        let test = md5::compute(format!("{}{}", id, i));
        let hash = format!("{:x}", test);

        if i % 100000000 == 0 && i != 0 {
            println!("answer not found int 100M iterations");
            break;
        }

        if &hash[0..5] == "00000" {
            if hashes.len() != 8 {
                hashes.push(hash.chars().nth(5).unwrap());
            }

            let pos = hash.chars().nth(5).unwrap();
            let pos = pos.to_digit(10);
            if pos != None && pos.unwrap() < 8 && hash2[pos.unwrap() as usize] == 'Z' {
                hash2[pos.unwrap() as usize] = hash.chars().nth(6).unwrap()
            }
        }
        i += 1;
    }
    (hashes.into_iter().collect(), hash2.iter().collect())
}

fn check(a: [char; 8]) -> u32 {
    let mut count = 0;
    for c in a {
        if c != 'Z' {
            count += 1;
        }
    }
    count
}
