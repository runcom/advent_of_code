extern crate regex;
use regex::Regex;
use std::collections::BTreeMap;
use std::cmp::Ordering;

fn main() {
    let path = include_str!("../../input/day4");
    let rooms = path.split('\n');

    let mut acc = 0;
    for room in rooms {
        if room == "" {
            continue;
        }
        acc += parts(room);
    }
    print!("part 1: {}\n", acc)
}

#[derive(Debug)]
#[derive(Eq)]
struct Pair {
    x: u8,
    y: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        if self.y.cmp(&other.y) == Ordering::Equal {
            if self.x < other.x {
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
        self.y.cmp(&other.y)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Pair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Pair) -> bool {
        self.y == other.y && self.x == other.x
    }
}

fn parts(r: &str) -> i32 {
    let re = Regex::new(r"(?P<roomid>.*)-(?P<secid>\d+)\[(?P<checksum>.*)\]").unwrap();
    let caps = re.captures(r).unwrap();
    let mut letters = caps.name("roomid").unwrap().replace("-", "").into_bytes();
    let dec = decrypt(caps.name("roomid").unwrap(),
                      caps.name("secid").unwrap().parse::<i32>().unwrap());
    if dec == "northpole object storage" {
        print!("part2: {}\n", caps.name("secid").unwrap())
    }
    letters.sort();
    let mut map = BTreeMap::new();
    let mut vec = Vec::new();
    for l in &letters {
        *map.entry(l).or_insert(0) += 1;
    }
    for (k, v) in &map {
        vec.push(Pair { x: **k, y: *v });
    }
    vec.sort();
    vec.reverse();
    let mut check = String::from("");
    for v in 0..5 {
        check.push(vec[v].x as char);
    }
    unsafe {
        let mut checksum = String::from(caps.name("checksum").unwrap());
        let c = checksum.as_mut_vec();
        let mut b = true;
        for i in 0..5 {
            if c[i] != check.as_bytes()[i] {
                b = false;
                break;
            }
        }
        if b {
            return caps.name("secid").unwrap().parse::<i32>().unwrap();
        } else {
            return 0;
        }
    }
}

fn decrypt(r: &str, id: i32) -> String {
    let mut s = String::from("");
    for c in r.chars() {
        if c == '-' {
            s.push(' ');
            continue;
        }
        let mut z = c as u8;
        for _ in 0..id {
            if z == 122 {
                z = 97;
                continue;
            }
            z += 1;
        }
        s.push(z as char)
    }
    s
}
