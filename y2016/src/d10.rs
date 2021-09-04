use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
struct Robot {
    low: String,
    high: String,
    stored: [usize; 2],
}

impl Robot {
    fn store(&mut self, v: usize) {
        if self.stored[0] == 0 {
            self.stored[0] = v;
        } else {
            self.stored[1] = v;
        }
    }

    fn reset(&mut self) {
        self.stored = [0, 0];
    }
}

pub fn ten(input: Vec<String>) -> (String, String) {
    let re_bot = Regex::new(r"(bot \d+) gives low to (\w+ \d+) and high to (\w+ \d+)").unwrap();
    let re_value = Regex::new(r"value (\d+) goes to (\w+ \d+)").unwrap();
    let mut robots = HashMap::new();

    // create mappings
    for l in re_bot.captures_iter(&input[0]) {
        let robot = Robot {
            low: l[2].to_string(),
            high: l[3].to_string(),
            stored: [0, 0],
        };
        robots.insert(l[1].to_string(), robot);
    }

    // fill in initial values
    for l in re_value.captures_iter(&input[0]) {
        let robot = robots.get_mut(&l[2].to_string()).unwrap();
        robot.store(l[1].parse().unwrap());
    }

    let mut out1 = "".to_string();

    // loop through until values passed through
    'outer: loop {
        let original = robots.clone();
        for (key, value) in original {
            if value.stored[0] != 0 && value.stored[1] != 0 {
                if (value.stored[0] == 61 && value.stored[1] == 17)
                    || (value.stored[0] == 17 && value.stored[1] == 61)
                {
                    out1 = key.clone();
                }

                let v: [usize; 2] = if value.stored[0] > value.stored[1] {
                    [value.stored[0], value.stored[1]]
                } else {
                    [value.stored[1], value.stored[0]]
                };

                let robot = robots.entry(value.high).or_insert(Robot {
                    low: "".to_string(),
                    high: "".to_string(),
                    stored: [0; 2],
                });
                robot.store(v[0]);

                let robot = robots.entry(value.low).or_insert(Robot {
                    low: "".to_string(),
                    high: "".to_string(),
                    stored: [0; 2],
                });
                robot.store(v[1]);

                robots.get_mut(&key).unwrap().reset();

                continue 'outer; // continue loop through keys if there is a double found
            }
        }
        break 'outer; // if no doubles found, exit
    }

    let mut out2 = 1;
    for v in ["output 0", "output 1", "output 2"] {
        out2 *= robots.get(&v.to_string()).unwrap().stored[0];
    }

    (out1, out2.to_string())
}
