use clap::{Arg,App,SubCommand};

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

    let mut msg = matches.value_of("input").unwrap();
    let s: Vec<char> = msg.chars().collect();
    for i in 1..26{
        print_chars(ciper(&s, i));
    }
}


fn ciper(msg: &Vec<char>,shift: usize) -> Vec<char>{
    let upper: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lower: &str = "abcdefghijklmnopqrstuvwxyz";
    
    let mut s = msg.clone();
    let mut res = Vec::new();

    let shift = shift % 26;
    for c in s{
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
            match upper.chars().position(|t| t == c){
                Some(pos) => {
                    let pos = shift + pos;
                    let newpos = {
                        if 26 <= pos {pos - 26} else {pos}
                    };
                    res.push(upper.chars().nth(newpos).unwrap());
                },
                None => {
                    res.push(c);
                }
            }
        }else{
            match lower.chars().position(|t| t == c){
                Some(pos) => {
                    let pos = shift + pos;
                    let newpos = {
                        if 26 <= pos {pos - 26} else {pos}
                    };
                    res.push(lower.chars().nth(newpos).unwrap());
                },
                None => {
                    res.push(c);
                }
            }
        
        }
    }
    res
}

fn print_chars(s: Vec<char>){
    for i in s{
        print!("{}",i);
    }
    print!("\n");
}