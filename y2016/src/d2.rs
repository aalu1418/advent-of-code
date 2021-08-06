pub fn two(input: Vec<String>) -> (String, String) {
    let input = &input[0];

    let s_pad = [
        ["1", "2", "3"].to_vec(),
        ["4", "5", "6"].to_vec(),
        ["7", "8", "9"].to_vec(),
    ];
    let d_pad = [
        ["", "", "1", "", ""].to_vec(),
        ["", "2", "3", "4", ""].to_vec(),
        ["5", "6", "7", "8", "9"].to_vec(),
        ["", "A", "B", "C", ""].to_vec(),
        ["", "", "D", "", ""].to_vec(),
    ];

    fn run(pad: Vec<Vec<&str>>, input: &String) -> String {
        let mut start = [1, 1];
        let mut code = Vec::new();

        for line in input.split("\n") {
            for d in line.split("") {
                let mut temp = start;
                let step: [i32; 2] = match d {
                    "L" => [-1, 0],
                    "R" => [1, 0],
                    "U" => [0, -1],
                    "D" => [0, 1],
                    _ => [0, 0],
                };
                temp[0] += step[0];
                temp[1] += step[1];

                for i in 0..start.len() {
                    temp[i] = if temp[i] < 0 { 0 } else { temp[i] };
                    temp[i] = if temp[i] > (pad.len() - 1) as i32 {
                        (pad.len() - 1) as i32
                    } else {
                        temp[i]
                    };
                }

                start = if pad[temp[1] as usize][temp[0] as usize] == "" {
                    start
                } else {
                    temp
                };
            }
            code.push(pad[start[1] as usize][start[0] as usize]);
        }
        code.join("")
    }

    (run(s_pad.to_vec(), input), run(d_pad.to_vec(), input))
}
