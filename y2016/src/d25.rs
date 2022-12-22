pub fn twentyfive(input: Vec<String>) -> (String, String) {
    let input = &input[0];

    // values of A must follow the pattern for B to be 0..1..0..1..0..
    // 0..1..2..5..10..21..
    let mut x = 0;
    let mut i = 0;
    while x < 231 * 11 {
        i += 1;
        x = 2 * x + i % 2;
    }

    let res = x - 231 * 11;

    // let b0 = crate::d12::run(&input, res, 0);
    (res.to_string(), "".to_string())
}

// fn algorithm(mut b: [isize; 4]) -> isize {
//     // cpy a d
//     // cpy 11 c
//     // cpy 231 b
//     // inc d
//     // dec b
//     // jnz b -2
//     // dec c
//     // jnz c -5
//     b = [b[0], 0, 0, b[0] + 231 * 11];

//     println!("ALGORITHM:   {:?}", b);

//     return b[0];
// }

// cpy d a
// jnz 0 0
// cpy a b
// cpy 0 a

// ----------- b alternate even and odd
// ----------- a alternate %2 == 0 || 1 when divided by 2..4..8..16
// cpy 2 c
// jnz b 2
// jnz 1 6
// dec b
// dec c
// jnz c -4
// inc a
// jnz 1 -7

// ---------- 2 - c = 1 || 2
// cpy 2 b
// jnz c 2
// jnz 1 4
// dec b
// dec c
// jnz 1 -4
// jnz 0 0

// out b

// jnz a -19 // if a != 0, rerun
// jnz 1 -21 // if a = 0, reset
