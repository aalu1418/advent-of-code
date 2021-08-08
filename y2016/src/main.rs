use std::env;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

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
}
