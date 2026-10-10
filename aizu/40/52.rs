fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
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
    let n = getline().trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [p, q] = ints[..] else { panic!() };
        let an = 180 * q;
        let ad = p;
        println!("{}", an / gcd(an, ad) - 1);
    }
}
