fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn f(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        let r = x % 10;
        ans = 10 * ans + r;
        x /= 10;
    }
    ans
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [mut x, mut y] = ints[..] else { panic!() };
    for _ in 0..8 {
        let tmp = f(x + y);
        x = y;
        y = tmp;
    }
    println!("{y}");
}
