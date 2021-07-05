fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn main() {
    let n: i64 = getline().trim().parse().ok().unwrap();
    let mut l = 1;
    for i in 2..n + 1 {
        let g = gcd(l, i);
        l = l / g * i;
    }
    println!("{}", l + 1);
}
