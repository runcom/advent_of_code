fn main() {
    let path = include_str!("input/day2");
    let steps = path.trim().split('\n');

    let pad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut pos1 = 1;
    let mut pos2 = 2;

    for step in steps {
        for c in step.chars() {
            match c {
                'R' if pos2 < 2 => pos2 += 1,
                'L' if pos2 > 0 => pos2 -= 1,
                'U' if pos1 > 0 => pos1 -= 1,
                'D' if pos1 < 2 => pos1 += 1,
                _ => {}
            }
        }
        print!("{}", pad[pos1][pos2]);
    }
    print!("\n")
}
