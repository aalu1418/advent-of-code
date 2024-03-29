use std::env;

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("⚠️  Not enough input arguments");
        return;
    }
    args.remove(0); // remove target arg

    let day = args[0].clone(); // clone day from array
    args.remove(0); // remove day argument

    let out = call(day, args);
    println!("Outputs: {:?}", out);
}

fn call(day: String, args: Vec<String>) -> (String, String) {
    match &day[..] {
        "1" => d1::one(args),
        "2" => d2::two(args),
        "3" => d3::three(args),
        "4" => d4::four(args),
        "5" => d5::five(args),
        "6" => d6::six(args),
        "7" => d7::seven(args),
        "8" => d8::eight(args),
        "9" => d9::nine(args),
        "10" => d10::ten(args),
        "11" => d11::eleven(args),
        "12" => d12::twelve(args),
        "13" => d13::thirteen(args),
        "14" => d14::fourteen(args),
        "15" => d15::fifteen(args),
        "16" => d16::sixteen(args),
        "17" => d17::seventeen(args),
        "18" => d18::eighteen(args),
        "19" => d19::nineteen(args),
        "20" => d20::twenty(args),
        "21" => d21::twentyone(args),
        "22" => d22::twentytwo(args),
        "23" => d23::twentythree(args),
        "24" => d24::twentyfour(args),
        "25" => d25::twentyfive(args),
        _ => not_implemented(day),
    }
}

fn not_implemented(day: String) -> (String, String) {
    println!("⚠️  Function '{}' not implemented", day);
    ("ERROR".to_string(), "ERROR".to_string())
}

#[cfg(test)]
mod tests {
    use super::call;
    use std::fs;
    use std::thread;

    const STACK_SIZE: usize = 1024 * 1024 * 1024;

    fn read_input(filename: &String) -> Vec<String> {
        let filepath = format!("./testdata/{}.txt", filename);
        let msg = format!("Error reading {}", filepath);
        let mut contents = fs::read_to_string(filepath).expect(&msg);
        contents.pop(); // remove line at end of file
        let mut out = Vec::new();
        out.push(contents);
        out
    }

    fn run(num: &str) -> (String, String) {
        let file = num.to_string();
        let input = read_input(&file);
        call(file, input)
    }

    fn ans(a: &str, b: &str) -> (String, String) {
        (a.to_string(), b.to_string())
    }

    #[test]
    fn day_1_check() {
        assert_eq!(run("1"), ans("146", "131"));
    }

    #[test]
    fn day_2_check() {
        assert_eq!(run("2"), ans("44558", "6BBAD"));
    }

    #[test]
    fn day_3_check() {
        assert_eq!(run("3"), ans("983", "1836"));
    }

    #[test]
    fn day_4_check() {
        assert_eq!(run("4"), ans("409147", "991"));
    }

    #[test]
    fn day_5_check() {
        assert_eq!(run("5"), ans("f77a0e6e", "999828ec"));
    }

    #[test]
    fn day_6_check() {
        assert_eq!(run("6"), ans("cyxeoccr", "batwpask"));
    }

    #[test]
    fn day_7_check() {
        assert_eq!(run("7"), ans("110", "242"));
    }

    #[test]
    fn day_8_check() {
        assert_eq!(run("8"), ans("106", " ##  #### #    #### #     ##  #   #####  ##   ### \n#  # #    #    #    #    #  # #   ##    #  # #    \n#    ###  #    ###  #    #  #  # # ###  #    #    \n#    #    #    #    #    #  #   #  #    #     ##  \n#  # #    #    #    #    #  #   #  #    #  #    # \n ##  #    #### #### ####  ##    #  #     ##  ###  \n"));
    }

    #[test]
    fn day_9_check() {
        assert_eq!(run("9"), ans("98135", "10964557606"));
    }

    #[test]
    fn day_10_check() {
        assert_eq!(run("10"), ans("bot 93", "47101"));
    }

    #[test]
    fn day_12_check() {
        assert_eq!(run("12"), ans("318007", "9227661"));
    }
    #[test]
    fn day_13_check() {
        assert_eq!(run("13"), ans("92", "124"));
    }
    #[test]
    fn day_14_check() {
        assert_eq!(run("14"), ans("23769", "20606"));
    }
    #[test]
    fn day_15_check() {
        assert_eq!(run("15"), ans("148737", "2353212"));
    }
    #[test]
    fn day_16_check() {
        assert_eq!(run("16"), ans("10010101010011101", "01100111101101111"));
    }
    #[test]
    fn day_17_check() {
        // requires larger stack size for recursion
        let child = thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(|| assert_eq!(run("17"), ans("RDULRDDRRD", "752")))
            .unwrap();

        child.join().unwrap();
    }
    #[test]
    fn day_18_check() {
        assert_eq!(run("18"), ans("2005", "20008491"));
    }
    #[test]
    fn day_19_check() {
        assert_eq!(run("19"), ans("1842613", "1424135"));
    }
    #[test]
    fn day_20_check() {
        assert_eq!(run("20"), ans("32259706", "113"));
    }
    #[test]
    fn day_21_check() {
        assert_eq!(run("21"), ans("agcebfdh", "afhdbegc"));
    }
    #[test]
    fn day_22_check() {
        assert_eq!(run("22"), ans("888", "236"));
    }
    #[test]
    fn day_23_check() {
        assert_eq!(run("23"), ans("12315", "479008875"));
    }
    #[test]
    fn day_24_check() {
        assert_eq!(run("24"), ans("470", "720"));
    }
    #[test]
    fn day_25_check() {
        assert_eq!(run("25"), ans("189", ""));
    }
}
