fn main() {
    let path = include_str!("input/day2");
    let steps = path.trim().split('\n');

    let pad = [[0, 0, 1, 0, 0],
               [0, 2, 3, 4, 0],
               [5, 6, 7, 8, 9],
               [0, 'A' as u8, 'B' as u8, 'C' as u8, 0],
               [0, 0, 'D' as u8, 0, 0]];
    let mut pos1 = 2;
    let mut pos2 = 0;

    for step in steps {
        for c in step.chars() {
            match c {
                'R' if pos2 < 4 && pad[pos1][pos2 + 1] != 0 => pos2 += 1,
                'L' if pos2 > 0 && pad[pos1][pos2 - 1] != 0 => pos2 -= 1,
                'U' if pos1 > 0 && pad[pos1 - 1][pos2] != 0 => pos1 -= 1,
                'D' if pos1 < 4 && pad[pos1 + 1][pos2] != 0 => pos1 += 1,
                _ => {}
            }
        }
        if pad[pos1][pos2] > 9 {
            print!("{}", pad[pos1][pos2] as char);
        } else {
            print!("{}", pad[pos1][pos2]);
        }
    }
    print!("\n")
}
