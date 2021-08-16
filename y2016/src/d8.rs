use regex::Regex;

pub fn eight(args: Vec<String>) -> (String, String) {
    let input = &args[0];
    let re = Regex::new(r"\d+").unwrap();
    let re_rotate = Regex::new(r"[xy]=").unwrap();
    let mut map: [[&str; 50]; 6] = [[" "; 50]; 6];

    for l in input.split("\n") {
        let val: Vec<usize> = re
            .find_iter(l)
            .filter_map(|d| d.as_str().parse().ok())
            .collect();

        let dir: Vec<String> = re_rotate
            .find_iter(l)
            .filter_map(|d| d.as_str().parse().ok())
            .collect();

        if dir.len() > 0 {
            let axis = dir[0].replace("=", "");
            match &axis[..] {
                "y" => map[val[0]].rotate_right(val[1]),
                "x" => {
                    let mut col: [&str; 6] = [" "; 6];
                    for i in 0..col.len() {
                        col[i] = map[i][val[0]];
                    }
                    col.rotate_right(val[1]);
                    for i in 0..col.len() {
                        map[i][val[0]] = col[i];
                    }
                }
                _ => {
                    println!("non expected type found");
                    return ("ERROR".to_string(), "ERROR".to_string());
                }
            }
            continue;
        }

        for i in 0..val[0] {
            for j in 0..val[1] {
                map[j][i] = "#";
            }
        }
    }

    let mut count = 0;
    let mut word: Vec<&str> = Vec::new();
    for row in map.iter() {
        count += row.iter().filter(|&&v| v == "#").count();
        for c in row.iter() {
            word.push(c);
        }
        word.push("\n");
    }
    let output: String = word.into_iter().collect();
    println!("{}", output);
    (count.to_string(), output)
}
