// naive solution (second answer takes a while to run)
pub fn twentythree(input: Vec<String>) -> (String, String) {
    let input = &input[0];

    let b0 = crate::d12::run(&input, 7, 0);
    // let b1 = crate::d12::run(&input, 12, 0);

    let a0 = algorithm([7, 0, 0, 0]);
    if a0.to_string() != b0 {
        println!("ERROR: patterned algorithm does not match brute force answer");
    }
    (b0, algorithm([12, 0, 0, 0]).to_string())
}

fn algorithm(mut b: [isize; 4]) -> isize {
    // cpy a b
    // dec b
    b = [b[0], b[0] - 1, 0, 0];

    let i = b[1];
    for _ in 0..i {
        // cpy a d
        // cpy 0 a
        // cpy b c
        // inc a
        // dec c
        // jnz c -2
        // dec d
        // jnz d -5
        // dec b
        // cpy b c
        // cpy c d
        // dec d
        // inc c
        // jnz d -2
        b = [b[0] * b[1], b[1] - 1, 2 * (b[1] - 1), 0];
    }

    // handle end condition
    // tgl c
    // cpy -16 c
    // jnz 1 c       -> cpy 1 c
    // cpy 75 c
    // jnz 97 d      -> cpy 97 d
    b[1] = 1;
    b[2] = 75;
    b[3] = 97;

    // inc a
    // inc d         -> dec d
    // jnz d -2
    // inc c         -> dec c
    // jnz c -5
    b[0] = b[0] + b[2] * b[3];
    b[2] = 0;
    b[3] = 0;
    println!("ALGORITHM:   {:?}", b);

    return b[0];
}
