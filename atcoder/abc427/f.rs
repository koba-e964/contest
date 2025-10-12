use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn en(a: &[i64], m: i64) -> HashMap<i64, u64> {
    let mut hm = HashMap::new();
    fn dfs(a: &[i64], x: i64, m: i64, hm: &mut HashMap<i64, u64>) {
        if a.is_empty() {
            let e = hm.entry(x).or_insert(0);
            *e += 1;
            return;
        }
        dfs(&a[1..], x, m, hm);
        if a.len() >= 2 {
            dfs(&a[2..], (x + a[0]) % m, m, hm);
        } else {
            dfs(&a[1..], (x + a[0]) % m, m, hm);
        }
    }
    dfs(a, 0, m, &mut hm);
    hm
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let m = ints[1];
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    if n <= 5 {
        let c = en(&a, m);
        println!("{}", c.get(&0).cloned().unwrap_or(0));
        return;
    }
    let c11 = en(&a[..n / 2 - 1], m);
    let c12 = en(&a[..n / 2], m);
    let c21 = en(&a[n / 2 + 2..], m);
    let c22 = en(&a[n / 2 + 1..], m);
    let mut ans = 0;
    for (k, v) in c12 {
        ans += c22.get(&((m - k) % m)).cloned().unwrap_or(0) * v;
    }
    let mid = a[n / 2];
    for (k, v) in c11 {
        ans += c21.get(&((2 * m - mid - k) % m)).cloned().unwrap_or(0) * v;
    }
    println!("{ans}");
}
