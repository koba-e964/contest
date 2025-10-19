fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [s, a, b, x] = ints[..] else { panic!() };
    let q = x / (a + b);
    let mut r = q * a;
    r += a.min(x % (a + b));
    println!("{}", s * r);
}
