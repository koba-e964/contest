fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [n, mut r] = ints[..] else { panic!() };
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [d, a] = ints[..] else { panic!() };
        let range = if d == 1 { 1600..2800 } else { 1200..2400 };
        if range.contains(&r) {
            r += a;
        }
    }
    println!{"{r}"};
}
