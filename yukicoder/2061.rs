fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

// https://yukicoder.me/problems/no/2061 (3)
// ビットごとに独立に数えられる。
// つまり ある a, a' に対し X = a * 2^{i+1} + 2^i + a' と X = a * 2^{i+1} + a' で差があるのであれば、
// 任意の b, b' に対して差がある。
// つまり、X = 0 と X = 2^i で差があるかどうか見て、差があったら答えを 2 倍する。
fn main() {
    getline();
    let mut a: Vec<i64> = getline()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    a.sort();
    const MOD: i64 = 998_244_353;
    let mut ans: i64 = 1;
    for i in 0..30 {
        let mut acp = a.clone();
        acp.sort_by_key(|&x| x ^ (1 << i));
        if a != acp {
            ans = ans * 2 % MOD;
        }
    }
    println!("{ans}");
}
