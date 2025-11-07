fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn f(c: i64, x: i64) -> i64 {
    if c == x {
        return 1 << 30;
    }
    if (c ^ x) > x {
        return c ^ x;
    }
    if c < x {
        return -1;
    }
    if (c - x) % 2 != 0 {
        return -1;
    }
    let s = (c - x) / 2;
    if (s & c) != s {
        return -1;
    }
    1 << 30 | s
}

fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [c, x] = ints[..] else { panic!() };
        println!("{}", f(c, x));
    }
}
