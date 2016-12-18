fn turn(direction: &mut [i32; 2], side: &str) {
    if side == "R" {
        *direction = [direction[1], -direction[0]];
    } else if side == "L" {
        *direction = [-direction[1], direction[0]];
    }
}

fn main() {
    use std::collections::HashSet;
    let path = include_str!("input/day1");
    let steps = path.split(',').map(|x| x.trim());

    let mut pos = [0, 0];
    let mut direction = [0, 1];
    let mut set = HashSet::new();
    set.insert(pos);
    let mut solved2 = false;

    for step in steps {
        let (side, n) = step.split_at(1);
        turn(&mut direction, side);
        let mut n = n.parse::<i32>().unwrap();
        while n > 0 {
            pos[0] += direction[0];
            pos[1] += direction[1];
            n -= 1;
            if !solved2 {
                if set.contains(&pos) {
                    println!("Part 2: {}", pos[0].abs() + pos[1].abs());
                    solved2 = true;
                } else {
                    set.insert(pos);
                }
            }
        }
    }
    println!("Part 1: {}", pos[0].abs() + pos[1].abs());
}
