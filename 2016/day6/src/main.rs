use std::collections::BTreeMap;
use std::cmp::Ordering;

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

fn main() {
    let path = include_str!("../../input/day6");

    for i in 0..8 {
        let msgs = path.split('\n');
        let mut map = BTreeMap::new();
        let mut vec = Vec::new();
        for x in msgs {
            if x == "" {
                continue;
            }
            *map.entry(x.as_bytes()[i as usize]).or_insert(0) += 1;
        }
        for (k, v) in &map {
            vec.push(Pair {
                x: *k as u8,
                y: *v,
            });
        }
        vec.sort();
        print!("part2: {:?}\n", vec[0].x as char);
        vec.reverse();
        print!("part1: {:?}\n", vec[0].x as char);
    }
}
