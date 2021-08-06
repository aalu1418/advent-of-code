use regex::Regex;

pub fn three(input: Vec<String>) -> (String, String) {
    let re = Regex::new(r"(\d+)[ ]+(\d+)[ ]+(\d+)[ ]*[\n]?").unwrap();
    let mut count = 0;
    let mut cols: [Vec<i32>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    for l in re.captures_iter(&input[0]) {
        let mut t: [i32; 3] = [
            l[1].parse().unwrap(),
            l[2].parse().unwrap(),
            l[3].parse().unwrap(),
        ];

        for i in 0..t.len() {
            cols[i].push(t[i])
        }

        t.sort();
        if t[0] + t[1] > t[2] {
            count += 1;
        }
    }

    let mut count2 = 0;
    for i in 0..cols[0].len() / 3 {
        for j in 0..cols.len() {
            let n = 3 * i;
            let mut t = [cols[j][n], cols[j][n + 1], cols[j][n + 2]];
            t.sort();

            if t[0] + t[1] > t[2] {
                count2 += 1;
            }
        }
    }

    println!("{:?}", cols[0]);
    (count.to_string(), count2.to_string())
}
