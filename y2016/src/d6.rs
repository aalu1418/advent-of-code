pub fn six(input: Vec<String>) -> (String, String) {
    let mut out1 = Vec::new();
    let mut out2 = Vec::new();

    let str: Vec<char> = input[0].chars().collect();
    for i in 0..8 {
        let mut chars: Vec<char> = str.iter().skip(i).step_by(9).copied().collect();
        println!("{:?}", chars);
        chars.sort();
        let original = chars.clone();
        chars.dedup();

        chars.sort_by(|a, b| {
            original
                .clone()
                .into_iter()
                .filter(|c| c == b)
                .count()
                .cmp(&original.clone().into_iter().filter(|c| c == a).count())
        });

        out1.push(chars[0]);
        out2.push(chars.last().copied().unwrap());
    }

    (out1.into_iter().collect(), out2.into_iter().collect())
}
