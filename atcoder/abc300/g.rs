use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn psieve(n: usize) -> Vec<bool> {
    if n <= 1 {
        return vec![false; n + 1];
    }
    let mut pr = vec![true; n + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..n + 1 {
        if !pr[i] { continue; }
        for j in 2..n / i + 1 {
            pr[i * j] = false;
        }
    }
    pr
}

fn rec(ps: &[i64], idx: usize, mut x: i64, memo: &mut HashMap<(usize, i64), i64>) -> i64 {
    if idx >= ps.len() {
        return 1;
    }
    if let Some(&val) = memo.get(&(idx, x)) {
        return val;
    }
    let oldx = x;
    let mut val = 0;
    while x >= 1 {
        val += rec(ps, idx + 1, x, memo);
        x /= ps[idx];
    }
    memo.insert((idx, oldx), val);
    val
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [n, p] = ints[..] else { panic!() };
    let p = p as usize;
    let tbl = psieve(p);
    let mut ps = vec![];
    for i in 2..p + 1 {
        if tbl[i] {
            ps.push(i as i64);
        }
    }
    ps.reverse();
    eprintln!("ps = {ps:?}");
    let mut memo = HashMap::new();
    println!("{}", rec(&ps, 0, n, &mut memo));
}
