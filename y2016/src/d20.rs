pub fn twenty(input: Vec<String>) -> (String, String) {
    let ranges = input[0].clone();

    let blocked = build(ranges);

    (
        find_lowest(&blocked).to_string(),
        unblocked_count(&blocked).to_string(),
    )
}

const MAX: usize = 4294967295;

fn build(r: String) -> Vec<(usize, usize)> {
    let mut parsed: Vec<(usize, usize)> = r
        .split('\n')
        .map(|x| {
            let v: Vec<&str> = x.split('-').collect();
            (v[0].parse().unwrap(), v[1].parse().unwrap())
        })
        .collect();
    parsed.sort_by(|a, b| a.0.cmp(&b.0));

    return parsed.into_iter().fold(Vec::new(), |mut ranges, val| {
        if ranges.len() == 0 {
            ranges.push(val);
        } else {
            let last = ranges.pop().unwrap();

            // determine if extension, overlap, separate
            if last.1 + 1 == val.0 || (val.0 < last.1 && val.1 >= last.1) {
                ranges.push((last.0, val.1));
            } else if val.0 < last.1 && val.1 < last.1 {
                ranges.push(last);
            } else {
                ranges.push(last);
                ranges.push(val);
            }
        }

        return ranges;
    });
}

fn find_lowest(v: &Vec<(usize, usize)>) -> usize {
    return v[0].1 + 1;
}

fn unblocked_count(v: &Vec<(usize, usize)>) -> usize {
    let mut count: usize = 0;

    for (i, pair) in v.iter().enumerate() {
        if i == v.len() - 1 {
            count += MAX - pair.1;
            continue;
        }

        if i == 0 && pair.0 != 0 {
            count += pair.0;
        }

        count += v[i + 1].0 - pair.1 - 1;
    }

    return count;
}
