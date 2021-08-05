pub fn two(input: Vec<String>) -> (String, String) {
    let input = &input[0];

    let pad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut start = [1, 1];
    let mut code1 = Vec::new();
    // let mut code2 = Vec::new();

    for line in input.split("\n") {
        for d in line.split("") {
            let step: [i32; 2] = match d {
                "L" => [-1, 0],
                "R" => [1, 0],
                "U" => [0, -1],
                "D" => [0, 1],
                _ => [0, 0],
            };
            start[0] += step[0];
            start[1] += step[1];

            for i in 0..start.len() {
                start[i] = if start[i] < 0 { 0 } else { start[i] };
                start[i] = if start[i] > 2 { 2 } else { start[i] };
            }
        }
        code1.push(pad[start[1] as usize][start[0] as usize].to_string());
    }

    (code1.join(""), "".to_string())
}
