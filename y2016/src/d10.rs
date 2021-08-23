use regex::Regex;
use std::collections::HashMap;

struct Robot {
    low: String,
    high: String,
    stored: [String; 2],
}

pub fn ten(input: Vec<String>) -> (String, String) {
    let re_bot = Regex::new(r"(bot \d+) gives low to (\w+ \d+) and high to (\w+ \d+)").unwrap();
    let re_value = Regex::new(r"(value \d+) goes to (\w+ \d+)").unwrap();
    let mut robots = HashMap::new();

    for l in re_bot.captures_iter(&input[0]) {
        let robot = Robot {
            low: l[2].to_string(),
            high: l[3].to_string(),
            stored: ["".to_string(), "".to_string()],
        };
        robots.insert(l[1].to_string(), robot);
    }

    for l in re_value.captures_iter(&input[0]) {
        let robot = robots.get_mut(&l[2].to_string()).unwrap();
        if robot.stored[0] == "" {
            robot.stored[0] = l[1].to_string();
        } else {
            robot.stored[1] = l[1].to_string();
        }

        let temp2 = robots.get(&l[2].to_string()).unwrap();
        println!("{} {} {:?}", temp2.low, temp2.high, temp2.stored);
    }

    ("".to_string(), "".to_string())
}
