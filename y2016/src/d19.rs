pub fn nineteen(input: Vec<String>) -> (String, String) {
    let count: usize = input[0].parse().unwrap();

    let mut elfs: Vec<usize> = (1..count + 1).collect();
    let mut elfs_two = elfs.clone();

    (
        reduce_left(&mut elfs, 0).to_string(),
        reduce_across(&mut elfs_two).to_string(),
    )
}

fn reduce_left(v: &mut Vec<usize>, start: usize) -> usize {
    if v.len() == 1 {
        return v[0];
    }

    let next_start = (start + v.len()) % 2; // preserve if on even or odd
    let mut ind = 0;
    v.retain(|_| {
        let save = ind % 2 == start;
        ind += 1;

        return save;
    });
    return reduce_left(v, next_start);
}

const KEEP_PATTERN: [bool; 3] = [false, false, true];

fn reduce_across(v: &mut Vec<usize>) -> usize {
    let mid = v.len() / 2;
    let mut removed = 0;
    let mut ind = 0;
    let keep_offset = (v.len() % 2) as usize;

    v.retain(|_| {
        let keep = ind < mid || KEEP_PATTERN[(ind - mid + keep_offset) % 3];

        ind += 1;
        if !keep {
            removed += 1;
        }
        return keep;
    });

    // end condition
    if v.len() == 1 {
        return v[0];
    }

    v.append(&mut v[0..removed].to_vec());
    v.drain(0..removed);

    return reduce_across(v);
}
