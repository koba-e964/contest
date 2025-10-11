fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let q = getline().trim().parse::<i32>().unwrap();
    let mut last = 0;
    let mut cur = 0;
    let mut c = vec![];
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [t, f] = ints[..] else { panic!() };
        if t > last {
            if last > 0 {
                c.push(last - cur);
            }
            cur = t;
            last = t + f;
        } else {
            last += f;
        }
    }
    c.push(last - cur);

    const W: usize = 4_000_000;
    let mut f = vec![0; W];
    let mut big = vec![];
    for &c in &c {
        if c >= W as i64 {
            big.push(c);
        } else {
            f[c as usize] += 1;
        }
    }
    let mut acc = vec![0; W + 1];
    for i in 0..W {
        acc[i + 1] = acc[i] + f[i];
    }
    const B: usize = 1000;
    let mut prec = vec![0; B];
    for i in 1..B {
        let mut ans = 0;
        for &c in &c {
            ans += c / i as i64;
        }
        prec[i] = ans;
    }
    for &s in &s {
        if s < B as i64 {
            println!("{}", prec[s as usize]);
        } else {
            let mut ans = 0;
            for i in 0..=W / s as usize {
                let hi = W.min((i + 1) * s as usize);
                ans += (acc[hi] - acc[i * s as usize]) * i as i64;
            }
            for &b in &big {
                ans += b / s;
            }
            println!("{ans}");
        }
    }
}
