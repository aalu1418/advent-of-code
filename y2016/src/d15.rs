use regex::Regex;

pub fn fifteen(input: Vec<String>) -> (String, String) {
    let re = Regex::new(
        r"Disc #(\d{1,}) has (\d{1,}) positions; at time=(\d{1,}), it is at position (\d{1,}).",
    )
    .unwrap();

    let mut data: Vec<[usize; 4]> = Vec::new();
    for cap in re.captures_iter(&input[0]) {
        let i: usize = cap[1].parse().unwrap(); // index
        let n: usize = cap[2].parse().unwrap(); // positions
        let t: usize = cap[3].parse().unwrap(); // time
        let x: usize = cap[4].parse().unwrap(); // position at time

        data.push([i, n, t, x]);
    }

    let mut data2 = data.clone();
    data2.push([data.len() + 1, 11, 0, 0]);

    (run(&data).to_string(), run(&data2).to_string())
}

fn run(data: &Vec<[usize; 4]>) -> usize {
    let mut i: usize = 0;
    'outer: loop {
        if i == 1_000_000_000 {
            println!("reached max loops");
            break;
        }
        let v = calculate(i, &data[0]);
        // println!("i = {} v = {} d = {:?}", i, v, &data[0]);

        for d in data {
            let val = calculate(i, d);
            // println!("i = {} v = {} d = {:?}", i, v, d);
            if v != val {
                i += 1;
                continue 'outer;
            }
        }

        break;
    }

    return i;
}

fn calculate(start: usize, data: &[usize; 4]) -> usize {
    let index = data[0];
    let steps = data[1];
    let init_position = data[3];

    return (start + index + init_position) % steps;
}
