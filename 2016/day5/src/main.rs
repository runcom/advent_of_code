extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let path = include_str!("../../input/day5");
    let input = path.trim();
    let mut done = false;
    let mut done2 = false;
    let mut i = 0;
    let mut res = String::from("");
    let mut res2 = vec!['_'; 8];
    let mut hasher = Md5::new();
    let mut pos_set = 0;

    while !done || !done2 {
        let s: String = format!("{}{}", input, i);
        hasher.input_str(&s);
        let hash = hasher.result_str();
        i += 1;
        if hash.starts_with("00000") {
            if !done {
                res.push(hash.as_bytes()[5] as char);
            }
            if !done2 {
                let index = hash.as_bytes()[5] as char;
                let idigit = index.to_digit(10);
                print!("{}", index);
                match idigit {
                    Some(id) => {
                        if id <= 7 && res2[id as usize] == '_' {
                            res2[id as usize] = hash.as_bytes()[6] as char;
                            pos_set += 1;
                        }
                    }
                    None => {}
                }
            }
        }
        hasher.reset();
        if res.len() == 8 {
            done = true;
        }
        if pos_set == 8 {
            done2 = true;
        }
        if done && done2 {
            break;
        }
    }
    print!("res1: {}\n", res);
    print!("res2: {:?}\n", res2);
}
