use combinations::Combinations;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Building {
    e: usize,
    floor: [Vec<String>; 4],
}

impl Building {
    fn init() -> Building {
        Building {
            e: 0,
            floor: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }

    fn end(&self) -> bool {
        let mut sum = 0;
        for f in &self.floor {
            sum += f.len();
        }
        sum == self.floor.last().unwrap().len()
    }

    fn valid(&self) -> bool {
        // for each floor
        for f in &self.floor {
            let m: Vec<&String> = f.iter().filter(|x| x.contains("m-")).collect(); // collect all chips
            let g: Vec<&String> = f.iter().filter(|x| x.contains("g-")).collect(); // coolect all generators

            // if more chips then generators and generators present, there will be unprotected chips
            if m.len() > g.len() && g.len() != 0 {
                return false;
            }

            // for each chip
            for chip in m {
                let count: Vec<&&String> = g.iter().filter(|x| x.contains(&chip[2..])).collect(); // check if generator exists for the chip
                                                                                                  // if no generator found, and generators exist return invalid
                if count.len() == 0 && g.len() != 0 {
                    return false;
                }
            }
        }
        return true;
    }

    fn combinations(&self) -> Vec<Vec<String>> {
        let mut combs: Vec<Vec<String>> = Vec::new();
        for i in 1..=2 {
            if self.floor[self.e].len() > i {
                combs.append(&mut Combinations::new(self.floor[self.e].clone(), i).collect());
            }
            if self.floor[self.e].len() == i {
                combs.push(self.floor[self.e].clone());
            }
        }
        return combs;
    }

    fn _sort(&mut self) {
        for i in 0..self.floor.len() {
            self.floor[i].sort();
        }
    }

    fn _move(&mut self, parts: Vec<String>, up: bool) -> bool {
        // only allow 1 or 2 inputs
        if parts.len() != 1 && parts.len() != 2 {
            return false;
        }

        let old = self.e.clone();
        if up {
            self.e += 1;
        } else {
            self.e -= 1;
        }

        for p in parts {
            // move parts
            let ind = self.floor[old].iter().position(|x| x[..] == p).unwrap();
            self.floor[self.e].push(p);
            self.floor[old].remove(ind);
        }

        self._sort();
        return true;
    }

    fn up(&mut self, parts: Vec<String>) -> bool {
        // return if at the top floor
        if self.e == self.floor.len() - 1 {
            return false;
        }

        return self._move(parts, true);
    }

    fn down(&mut self, parts: Vec<String>) -> bool {
        // return if at the bottom floor
        if self.e == 0 {
            return false;
        }
        return self._move(parts, false);
    }
}

pub fn eleven(input: Vec<String>) -> (String, String) {
    let input = &input[0];
    // Copy not implemented for Floor b/c string (so each one is implemented)
    let mut building = Building::init();

    let re_generator = Regex::new(r"(\w+) generator").unwrap();
    let re_microchip = Regex::new(r"(\w+)-compatible microchip").unwrap();

    let mut i = 0;
    for l in input.split("\n") {
        for g in re_generator.captures_iter(&l) {
            let mut str = "g-".to_string();
            str.push_str(&g[1]);
            building.floor[i].push(str);
        }
        for m in re_microchip.captures_iter(&l) {
            let mut str = "m-".to_string();
            str.push_str(&m[1]);
            building.floor[i].push(str);
        }
        i += 1;
    }
    building._sort();
    let out1 = search(building.clone());
    let new_components = vec![
        "g-elerium".to_string(),
        "m-elerium".to_string(),
        "g-dilithium".to_string(),
        "m-dilithium".to_string(),
    ];
    building.floor[0].extend(new_components);
    building._sort();
    // let out2 = search(building);

    (out1.to_string(), "".to_string())
}

fn search(building: Building) -> usize {
    let mut visited = HashMap::new();
    let mut current: Vec<Building> = Vec::new();
    visited.insert(building.clone(), true);
    current.push(building);
    let mut i = 0;
    'outer: loop {
        println!("Round {}: length {}", i, current.len());
        let mut next: Vec<Building> = Vec::new();

        // select the each instance from current
        for b in &current {
            // check if end condition met
            if b.end() {
                println!("End condition met!");
                break 'outer;
            }

            // find all moves
            let combs = b.combinations();
            for step in combs {
                let mut temp = b.clone();
                // check if move is valid & store to compare previous states check
                if temp.up(step.clone()) && temp.valid() && !visited.contains_key(&temp) {
                    visited.insert(temp.clone(), true);
                    next.push(temp);
                }
                let mut temp = b.clone();
                // check if move is valid & store to compare previous states check
                if temp.down(step) && temp.valid() && !visited.contains_key(&temp) {
                    visited.insert(temp.clone(), true);
                    next.push(temp);
                }
            }
        }
        if next.len() == 0 {
            println!("No options found. Last instance: {:?}", &current);
            break 'outer;
        }
        current = next;
        i += 1;
    }
    return i;
}
