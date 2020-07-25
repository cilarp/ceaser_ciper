use clap::{Arg,App};

fn main() {
    let matches = App::new("Ciper Crypto")
                        .version("1.0")
                        .author("cilarp. cilarp914@gmail.com")
                        .about("deode/encode ciper")
                        .arg(Arg::with_name("input")
                            .help("Set the input strings to use")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("shift")
                            .short("s")
                            .help("shift words"))
                        .get_matches();

    let msg = matches.value_of("input").unwrap();
    let s: Vec<char> = msg.chars().collect();
    for i in 1..26{
        print_chars(ciper(&s, i));
    }
}


fn ciper(msg: &Vec<char>,shift: usize) -> Vec<char>{
    let upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lower: &str = "abcdefghijklmnopqrstuvwxyz";
    
    let mut res = Vec::new();

    let shift = shift % 26;
    for c in msg.clone(){
        let c_ascii = c as u8;
        
        if c.is_whitespace() {
            res.push(c);
            continue;
        }

        //not in a-z or A-Z
        if c_ascii < 65 || (90 < c_ascii && c_ascii < 97) || 123 < c_ascii{
            res.push(c);
            continue;
        }

        if c.is_uppercase(){
            process(c, upper, shift, &mut res);
        }else{  
            process(c, lower, shift, &mut res);
        }
    }
    res
}

fn process(c: char,strings: &str,shift: usize,res: &mut Vec<char>){
    match strings.chars().position(|t| t == c){
        Some(pos) => {
            let pos = shift + pos;
            let newpos = {
                if 26 <= pos {pos - 26} else {pos}
            };
            res.push(strings.chars().nth(newpos).unwrap());
        },
        None => {
            res.push(c);
        }
    }
}

fn print_chars(s: Vec<char>){
    for i in s{
        print!("{}",i);
    }
    print!("\n");
}