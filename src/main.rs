use clap::{App, Arg};

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
    let s: Vec<char> = msg.chars().collect();
    if shift == "all" {
        for i in 1..26 {
            print_chars(ciper(&s, i));
        }
    } else if shift.contains("-") {
        let shift: Vec<&str> = shift.split("-").collect();
        let shift: Vec<usize> = shift
            .iter()
            .map(|s| s.parse().expect("input numbef before/after -  "))
            .collect();
        if shift.len() % 2 == 1 {
            let (start, end) = (shift[0], shift[shift.len() - 1]);
            for i in start..end + 1 {
                print_chars(ciper(&s, i));
            }
        } else {
            let times = shift.len() / 2;
            for i in 0..times {
                for j in shift[i * 2]..shift[i * 2 + 1] {
                    print_chars(ciper(&s, j));
                }
            }
        }
    } else {
        let shift: usize = match shift.parse() {
            Ok(s) => s,
            Err(e) => panic!("Input integer or \"all\" for shift"),
        };
        print!("Encode: ");
        print_chars(ciper(&s, shift));
        print!("Decode: ");
        print_chars(ciper(&s, 26 - shift));
    }
}

fn ciper(msg: &Vec<char>, shift: usize) -> Vec<char> {
    let upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lower: &str = "abcdefghijklmnopqrstuvwxyz";

    let mut res = Vec::new();

    let shift = shift % 26;
    for c in msg.clone() {
        let c_ascii = c as u8;

        if c.is_whitespace() {
            res.push(c);
            continue;
        }

        //not in a-z or A-Z
        if c_ascii < 65 || (90 < c_ascii && c_ascii < 97) || 123 < c_ascii {
            res.push(c);
            continue;
        }

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
