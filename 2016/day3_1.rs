fn main() {
    let path = include_str!("input/day3");
    let steps = path.split('\n');
    let mut acc = 0;

    for step in steps {
        let t = step.split_whitespace();
        let mut res = [0, 0, 0];
        let mut i = 0;
        for s in t {
            let n = s.trim().parse::<i32>().unwrap();
            res[i] = n;
            i += 1
        }
        if res[0] + res[1] >= res[2] && res[1] + res[2] > res[0] && res[2] + res[0] > res[1] {
            acc += 1;
        }
    }
    print!("{}\n", acc)
}
