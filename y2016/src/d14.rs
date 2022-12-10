use md5;

pub fn fourteen(input: Vec<String>) -> (String, String) {
    let salt = &input[0];

    (find(salt, generate), find(salt, generate2))
}

fn find(salt: &String, g: fn(&String, &isize) -> String) -> String {
    let mut i = 0;

    // generate first 1000
    let mut hashes: Vec<String> = Vec::new();
    for k in 0..1001 {
        hashes.push(g(salt, &k));
    }

    let mut found = 0;
    'outer: loop {
        let hash = hashes[0].clone();
        hashes.drain(0..1);
        let c = triple(&hash);

        if c != "" {
            // println!("found triple: {} {} {}", i, c, hash);
            let mut f = false;
            for h in &hashes {
                if h.contains(&vec![c.clone(); 5].join("")) {
                    // println!("validated: {} {} {}", i, c, h);
                    f = true;
                    break;
                }
            }

            if f {
                found += 1;

                // end condition
                if found == 64 {
                    break 'outer;
                }
            }
        }
        hashes.push(g(salt, &(i + 1001)));
        i += 1;
    }

    i.to_string()
}

fn generate(s: &String, i: &isize) -> String {
    format!("{:x}", md5::compute(format!("{}{}", s, i)))
}

fn generate2(s: &String, i: &isize) -> String {
    let mut h = generate(s, i);

    for _ in 0..2016 {
        h = format!("{:x}", md5::compute(h));
    }
    return h;
}

fn triple(s: &String) -> String {
    let s: Vec<&str> = s.split("").collect();
    let mut t = "";
    let mut n = 0;
    for (_, c) in s.iter().enumerate() {
        if t != *c {
            t = c;
            n = 1;
            continue;
        }
        n += 1;

        // end condition
        if n == 3 {
            return t.to_string();
        }
    }

    "".to_string()
}
