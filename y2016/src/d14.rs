use md5;

pub fn fourteen(input: Vec<String>) -> (String, String) {
    let salt = &input[0];
    let mut i = 0;

    // initialize first 1000
    let mut hashes: Vec<String> = Vec::new();
    for i in 0..1000 {
        let hash = format!("{:x}", md5::compute(format!("{}{}", salt, i)));
        hashes.push(hash);
    }

    let mut found = 0;
    'outer: loop {
        // end condition
        if found == 64 {
            break 'outer;
        }

        let test_key = hashes[0].clone();
        hashes.drain(0..1); // remove first instance from stack

        // generate hash to form next 1000 to check
        let hash = format!("{:x}", md5::compute(format!("{}{}", salt, i)));
        hashes.push(hash);

        // check if there's a triple
        let c = triple(&test_key);
        if &c != "" {
            // if there's a triple search the vec of 1000 to find match
            for h in &hashes {
                if h.contains(&vec![c.clone(); 5].join("")) {
                    println!("{} {} {}", i, c, h);
                    found += 1;
                    break;
                }
            }
        }

        i += 1;
    }

    (i.to_string(), "".to_string())
}

fn triple(s: &String) -> String {
    let s: Vec<&str> = s.split("").collect();
    let mut t = "";
    let mut n = 0;
    for (i, c) in s.iter().enumerate() {
        if t != *c {
            t = c;
            n = 1;
            continue;
        }
        n += 1;

        // end condition
        if n == 3 {
            // if at the end or the next char is not the same
            if i == s.len() - 1 || t != s[i + 1] {
                return t.to_string();
            }
        }
    }

    "".to_string()
}
