pub fn twentyone(input: Vec<String>) -> (String, String) {
    (scramble(input[0].clone()), unscramble(input[0].clone()))
}

fn scramble(input: String) -> String {
    let mut base = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for line in input.split('\n') {
        let data: Vec<&str> = line.split_whitespace().collect();

        match data[0] {
            "swap" => match data[1] {
                "position" => base.swap(data[2].parse().unwrap(), data[5].parse().unwrap()),
                "letter" => {
                    let ind_x = find_index(&base, data[2].chars().next().unwrap());
                    let ind_y = find_index(&base, data[5].chars().next().unwrap());

                    base.swap(ind_x, ind_y);
                }
                _ => panic!("invalid sub-type"),
            },
            "rotate" => match data[1] {
                "left" => base.rotate_left(data[2].parse().unwrap()),
                "right" => base.rotate_right(data[2].parse().unwrap()),
                "based" => {
                    let ind = find_index(&base, data[6].chars().next().unwrap());

                    let count = (ind + 1 + (ind >= 4) as usize) % base.len();
                    base.rotate_right(count)
                }
                _ => panic!("invalid sub-type"),
            },
            "reverse" => {
                let x: usize = data[2].parse().unwrap();
                let y: usize = data[4].parse().unwrap();

                base[x..y + 1].reverse()
            }
            "move" => {
                let c = base.remove(data[2].parse().unwrap());
                base.insert(data[5].parse().unwrap(), c)
            }
            _ => panic!("invalid type"),
        }
    }
    return base.iter().collect();
}

fn unscramble(input: String) -> String {
    let mut base: Vec<char> = "fbgdceah".chars().collect();
    // let mut base: Vec<char> = "agcebfdh".chars().collect();
    // let mut base: Vec<char> = "decab".chars().collect();
    let mut shift_vec: Vec<isize> = vec![0; base.len()];

    for i in 0..base.len() {
        let next = ((i + 1 + (i >= 4) as usize) + i) % base.len();

        shift_vec[next] = i as isize - next as isize;
    }

    println!("{:?}", shift_vec);

    for line in input.rsplit('\n') {
        let data: Vec<&str> = line.split_whitespace().collect();

        match data[0] {
            "swap" => match data[1] {
                "position" => base.swap(data[2].parse().unwrap(), data[5].parse().unwrap()),
                "letter" => {
                    let ind_x = find_index(&base, data[2].chars().next().unwrap());
                    let ind_y = find_index(&base, data[5].chars().next().unwrap());

                    base.swap(ind_x, ind_y);
                }
                _ => panic!("invalid sub-type"),
            },
            "rotate" => match data[1] {
                "left" => base.rotate_right(data[2].parse().unwrap()),
                "right" => base.rotate_left(data[2].parse().unwrap()),
                "based" => {
                    let ind = find_index(&base, data[6].chars().next().unwrap());

                    let shift = shift_vec[ind];

                    if shift < 0 {
                        base.rotate_left((shift * -1) as usize);
                    } else {
                        base.rotate_right(shift as usize);
                    }
                }
                _ => panic!("invalid sub-type"),
            },
            "reverse" => {
                let x: usize = data[2].parse().unwrap();
                let y: usize = data[4].parse().unwrap();

                base[x..y + 1].reverse()
            }
            "move" => {
                let c = base.remove(data[5].parse().unwrap());
                base.insert(data[2].parse().unwrap(), c)
            }
            _ => panic!("invalid type"),
        }
    }
    return base.iter().collect();
}

fn find_index(b: &Vec<char>, c: char) -> usize {
    b.iter().position(|&v| v == c).unwrap()
}
