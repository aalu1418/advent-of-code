pub fn eighteen(input: Vec<String>) -> (String, String) {
    let base: Vec<char> = input[0].chars().collect();

    (run(40, base.clone()), run(400000, base.clone()))
}

fn run(rows: usize, base: Vec<char>) -> String {
    let mut tiles: Vec<Vec<char>> = vec![base.clone()];
    let mut count = base
        .iter()
        .fold(0, |acc, x| if *x == '.' { acc + 1 } else { acc });

    loop {
        let (t, c) = generate(tiles.last().unwrap());

        tiles.push(t);
        count += c;

        if rows == tiles.len() {
            break;
        }
    }

    count.to_string()
}

fn generate(base: &Vec<char>) -> (Vec<char>, usize) {
    let mut out = base.clone();
    let mut count: usize = 0;

    for (i, _) in base.iter().enumerate() {
        let left = if i == 0 {
            false
        } else {
            if base[i - 1] == '^' {
                true
            } else {
                false
            }
        };

        let right = if i == base.len() - 1 {
            false
        } else {
            if base[i + 1] == '^' {
                true
            } else {
                false
            }
        };

        if left != right {
            out[i] = '^';
        } else {
            out[i] = '.';
            count += 1;
        }
    }
    (out, count)
}
