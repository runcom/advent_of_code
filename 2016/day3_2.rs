fn main() {
    let path = include_str!("input/day3");
    let mut steps = path.lines();
    let mut acc = 0;

    loop {
        match steps.next() {
            Some(line1) => {
                let line1 = get_triangle(line1);
                let line2 = get_triangle(steps.next().unwrap());
                let line3 = get_triangle(steps.next().unwrap());
                for i in 0..3 {
                    if line1[i] + line2[i] > line3[i] && line2[i] + line3[i] > line1[i] &&
                       line3[i] + line1[i] > line2[i] {
                        acc += 1;
                    }
                }
            }
            None => break,
        }
    }
    print!("{}\n", acc)
}

fn get_triangle(s: &str) -> Vec<i32> {
    return s.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}
