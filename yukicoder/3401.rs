fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// https://yukicoder.me/problems/no/3401 (3)
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = ints[0];
    let maxw = ints[1];
    let mut vw = vec![];
    let mut ma = (0, 0);
    for i in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        vw.push((ints[0], ints[1] as usize));
        let q = (ints[0] << 32) / ints[1];
        ma = std::cmp::max(ma, (q, i));
    }
    const W: usize = 20_000_000;
    let mut dp = vec![0i64; W];
    for &(v, w) in &vw {
        for i in 0..W - w {
            dp[i + w] = dp[i + w].max(dp[i] + v);
        }
    }
    let mut ans = 0;
    let (optv, optw) = vw[ma.1];
    for i in 0..W {
        if i <= maxw {
            ans = ans.max(dp[i] + ((maxw - i) / optw) as i64 * optv);
        }
    }
    println!("{ans}");
}
