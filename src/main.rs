use clap::{App, Arg};

//I referred to https://gist.github.com/RockyTV/c3412fabf6cec5156e49
//
//Usage
//cargo run [strings] [option]
//
//description
//You can use "all" or number or (number-number)+ as [option]
//If you choice "all" as option,then this program prints out all considerable results.
//If you choice number,then this program prints out encoded and decoded result shifting given string given number.
//If you choice (number-number)+ as option,then this program prints out results shifting given string given ranges.
//Also you can use ',' such like number-number,number-number

fn main() {
    let matches = App::new("Ciper Crypto")
        .version("1.0")
        .author("cilarp. cilarp914@gmail.com")
        .about("deode/encode ciper")
        .arg(
            Arg::with_name("input")
                .help("Set the input strings to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("shift")
                .help("shift words")
                .required(true)
                .index(2)
                .short("s")
                .long("shift"),
        )
        .get_matches();

    let msg = matches.value_of("input").unwrap();
    let shift = matches.value_of("shift").unwrap();
    let msg: Vec<char> = msg.chars().collect();
    if shift == "all" {
        for shift in 1..26 {
            print_ciper(&msg, shift);
        }
    } else if shift.contains("-") {
        if shift.contains(',') {
            let shifts: Vec<&str> = shift.split(',').collect();
            for shift in shifts {
                print_ciper_with_hyphen(shift, &msg);
            }
        } else {
            print_ciper_with_hyphen(shift, &msg);
        }
    } else {
        let shift: usize = match shift.parse() {
            Ok(shift) => shift,
            Err(e) => panic!("Input integer or \"all\" for shift\n{}", e),
        };
        print!("Encode: ");
        print_ciper(&msg, shift);
        print!("Decode: ");
        print_ciper(&msg, 26 - shift);
    }
}

fn ciper(msg: &Vec<char>, shift: usize) -> Vec<char> {
    let upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lower: &str = "abcdefghijklmnopqrstuvwxyz";

    let mut res = Vec::new();

    let shift = shift % 26;
    for c in msg.clone() {
        if c.is_uppercase() {
            process(c, upper, shift, &mut res);
        } else {
            process(c, lower, shift, &mut res);
        }
    }
    res
}

fn process(c: char, strings: &str, shift: usize, res: &mut Vec<char>) {
    match strings.chars().position(|t| t == c) {
        Some(pos) => {
            let pos = shift + pos;
            let newpos = {
                if 26 <= pos {
                    pos - 26
                } else {
                    pos
                }
            };
            res.push(strings.chars().nth(newpos).unwrap());
        }
        None => {
            res.push(c);
        }
    }
}

fn print_chars(s: Vec<char>) {
    for i in s {
        print!("{}", i);
    }
    print!("\n");
}

fn print_ciper(msg: &Vec<char>, shift: usize) {
    print_chars(ciper(&msg, shift));
}

fn print_ciper_with_hyphen(shift: &str, msg: &Vec<char>) {
    let shift: Vec<String> = shift
        .split("-")
        .map(|s| s.to_string().replace(" ", ""))
        .collect();
    let shifts: Vec<usize> = shift
        .iter()
        .map(|s| s.parse().expect("input numbef before/after -  "))
        .collect();

    if shifts.len() % 2 == 1 {
        let (start, end) = (shifts[0], shifts[shifts.len() - 1] + 1);
        for shift in start..end {
            print_ciper(&msg, shift);
        }
    } else {
        let times = shifts.len() / 2;
        for time in 0..times {
            for shift in shifts[time * 2]..shifts[time * 2 + 1] + 1 {
                print_ciper(&msg, shift);
            }
        }
    }
}
