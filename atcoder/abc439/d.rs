fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(a: &[i64]) -> i64 {
    let mut hm = std::collections::HashMap::new();
    let mut ans = 0;
    for &a in a {
        if a % 5 == 0 {
            let x = a / 5 * 7;
            let y = a / 5 * 3;
            if let Some(&xx) = hm.get(&x) {
                if let Some(&yy) = hm.get(&y) {
                    ans += xx * yy;
                }
            }
        }
        *hm.entry(a).or_insert(0i64) += 1;
    }
    ans
}

fn main() {
    getline();
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ans = calc(&a);
    a.reverse();
    ans += calc(&a);
    println!("{ans}");
}
