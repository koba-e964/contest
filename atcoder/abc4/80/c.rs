fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let k = ints[1];
    let t = ints[2];
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    a.sort();
    let mut ans = 0;
    let mut cur = 0;
    for a in a {
        let a = a.min(t);
        ans += (0.max(a - cur) + k - 1) / k;
        cur = a + k;
    }
    ans += (0.max(t - cur) + k - 1) / k;
    println!("{ans}");
}
