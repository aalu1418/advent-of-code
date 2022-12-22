use std::num::ParseIntError;

pub fn twentytwo(input: Vec<String>) -> (String, String) {
    let disks: Vec<((usize, usize), usize, usize, usize)> = input[0]
        .lines()
        .filter_map(|x| {
            let v: Vec<&str> = x.split_whitespace().collect();
            if v.len() < 5 {
                return None;
            }

            let name = v[0].split("-").collect::<Vec<&str>>();
            if name.len() != 3 {
                return None;
            }

            let x: usize = name[1].trim_start_matches('x').parse().unwrap();
            let y: usize = name[2].trim_start_matches('y').parse().unwrap();

            let size = match parse(v[1]) {
                Ok(v) => v,
                Err(_) => {
                    return None;
                }
            };
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

            Some(((x, y), size, used, avail))
        })
        .collect();

    (valid_pairs(&disks).to_string(), path(&disks).to_string())
}

fn parse(s: &str) -> Result<usize, ParseIntError> {
    let mut v = s.to_string();
    v.pop();
    v.parse()
}

fn valid_pairs(disks: &Vec<((usize, usize), usize, usize, usize)>) -> usize {
    let mut count = 0;

    for d in disks {
        count += disks
            .iter()
            .filter(|x| x.3 >= d.2 && x.0 != d.0 && d.2 != 0)
            .count();
    }

    return count;
}

fn path(disks: &Vec<((usize, usize), usize, usize, usize)>) -> usize {
    let mut grid: Vec<Vec<(usize, usize)>> =
        vec![vec![(0, 0); disks.last().unwrap().0 .1 + 1]; disks.last().unwrap().0 .0 + 1];

    let target = (disks.last().unwrap().0 .0, 0);
    let mut empty = (0, 0);
    for d in disks {
        grid[d.0 .0][d.0 .1] = (d.1, d.2);

        if d.2 == 0 {
            empty = (d.0 .0, d.0 .1); // assume only 1 empty partition
        }
    }

    // validate simplest path
    // (X, 0) -> (0, 0)
    // empty space moves to space in front of current -> current moves to empty -> empty moves around (U shape)
    // validate all involved coordinates will fit all chunks
    // validate wall location
    let mut min_size = 1_000_000;
    let mut max_used = 0;
    let mut wall_gap_x = 0;
    let mut wall_y = 0;
    let mut wall_count = 0;
    for (x, l) in grid.clone().into_iter().enumerate() {
        if l[0].0 < min_size {
            min_size = l[0].0
        }
        if l[1].0 < min_size {
            min_size = l[1].0
        }
        if l[0].1 > max_used {
            max_used = l[0].1
        }
        if l[1].1 > max_used {
            max_used = l[1].1
        }

        if min_size < max_used {
            println!(
                "ERROR: this algorithm will not work for min size: {} + max used: {}",
                min_size, max_used
            );
            return 0;
        }

        if wall_y == 0 {
            for (i, w) in l.clone().into_iter().enumerate() {
                if w.1 > 100 {
                    wall_y = i;
                    break;
                }
            }
        }

        if l[wall_y].1 > 100 {
            wall_count += 1
        } else {
            wall_gap_x = x;
        }
    }

    if wall_count != grid.len() - 1 {
        println!(
            "ERROR: wall with one hole not detected. Expected {} got {}",
            grid.len(),
            wall_count
        );
    }

    println!("WALL: detected in Y = {}, gap @ X = {}", wall_y, wall_gap_x);

    let steps = 5 * (target.0 - 1) + 1;
    println!(
        "once _ is @ {:?}, it will take 5(N - 1)+1 steps: {}",
        (target.0 - 1, 0),
        steps
    );

    let goal = (target.0 - 1, 0);
    println!(
        "EMPTY: {:?} -> {:?} -> {:?}",
        empty,
        (wall_gap_x, wall_y),
        goal
    );

    return steps
        + empty.0.abs_diff(wall_gap_x)
        + goal.0.abs_diff(wall_gap_x)
        + empty.1.abs_diff(goal.1);
}
