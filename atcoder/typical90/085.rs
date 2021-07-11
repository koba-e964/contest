fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: i64 = getline().trim().parse().unwrap();
    let mut divs = vec![];
    let mut d = 1;
    while d * d <= n {
        if n % d == 0 {
            divs.push(d);
            if d * d != n {
                divs.push(n / d);
            }
        }
        d += 1;
    }
    divs.sort();
    let mut ans = 0;
    for i in 0..divs.len() {
        let tmp = n / divs[i];
        for j in 0..i + 1 {
            if tmp % divs[j] != 0 {
                continue;
            }
            let q = n / divs[i] / divs[j];
            if q <= divs[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
