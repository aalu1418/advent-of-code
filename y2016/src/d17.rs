use md5;
use std::convert::TryInto;

pub fn seventeen(input: Vec<String>) -> (String, String) {
    let mut start: Vec<Path> = Vec::new();
    start.push(new_path(input[0].clone()));

    run(start, 0, Vec::new())
}

#[derive(Clone, Debug)]
struct Path {
    base: String,
    step: Vec<char>,
    x: usize,
    y: usize,
}

fn new_path(base: String) -> Path {
    Path {
        base,
        step: Vec::new(),
        x: 0,
        y: 0,
    }
}

impl Path {
    fn md5(&self) -> [char; 4] {
        let hash: Vec<char> = format!(
            "{:x}",
            md5::compute(format!(
                "{}{}",
                self.base,
                self.step.iter().collect::<String>()
            ))
        )[..4]
            .chars()
            .collect();
        hash.try_into().unwrap()
    }

    fn step(&mut self, d: char) -> bool {
        if (self.x == 0 && d == 'L')
            || (self.x == 3 && d == 'R')
            || (self.y == 0 && d == 'U')
            || (self.y == 3 && d == 'D')
        {
            return false;
        }

        self.step.push(d);
        match d {
            'L' => self.x -= 1,
            'R' => self.x += 1,
            'U' => self.y -= 1,
            'D' => self.y += 1,
            _ => (),
        }
        return true;
    }

    fn end(&self) -> bool {
        return self.x == 3 && self.y == 3;
    }
}

static DIRECTION: [char; 4] = ['U', 'D', 'L', 'R'];

fn run(mut stack: Vec<Path>, mut max: usize, mut min_step: Vec<char>) -> (String, String) {
    println!("{}", stack.len());
    // if stack.len() == 0 {
    if min_step.len() != 0 {
        return (min_step.iter().collect(), max.to_string());
    }

    // get first element
    let e: Path = stack.first().unwrap().clone();
    stack.drain(0..1);

    // find valid move directions
    let door_arr = e.md5();
    for (i, door) in door_arr.iter().enumerate() {
        match door {
            'b' | 'c' | 'd' | 'e' | 'f' => {
                // open door
                let mut n = e.clone();
                let valid_step = n.step(DIRECTION[i]);

                // if valid step (if not skip)
                if valid_step {
                    // check end condition for step
                    if n.end() {
                        if min_step.len() == 0 {
                            min_step = n.step.clone();
                        }
                        max = n.step.len();
                    } else {
                        // if not end, push to stack
                        stack.push(n);
                    }
                }
            }
            _ => (),
        }
    }

    return run(stack, max, min_step);
}
