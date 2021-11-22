use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn factorize(mut x: i64) -> Vec<(i64, usize)> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        ans.push((x, 1));
    }
    ans
}

fn dfs(v: usize, pe: &[(i64, usize)],
       mut x: i64, mut phi: i64, ans: &mut Vec<(i64, i64)>) {
    if v >= pe.len() {
        ans.push((x, phi));
        return;
    }
    let (p, e) = pe[v];
    dfs(v + 1, pe, x, phi, ans);
    x *= p;
    phi *= p - 1;
    for _ in 0..e {
        dfs(v + 1, pe, x, phi, ans);
        x *= p;
        phi *= p;
    }
}

fn get_divs_and_phis(x: i64) -> Vec<(i64, i64)> {
    let pe = factorize(x);
    let mut ans = vec![];
    dfs(0, &pe, 1, 1, &mut ans);
    ans
}

fn powmod(x: i64, mut e: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % MOD;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    sum
}

const MOD: i64 = 1_000_000_007;

// https://yukicoder.me/problems/no/377 (5)
// Burnside のでない定理。gcd(y, x) = a, 0 <= y < x を満たす y の個数は phi(x / a) なので、\sum_{a | H, b | W} phi(H/a) phi(W/b) K^{ab} / (HW) が答え。-> 間違い。(a, b) in Z/H * Z/W の周期は HW/(ab) ではなく lcm(H/a,W/b)。よってK^{HW/lcm(H/a,W/b)} を足す必要がある。
fn main() {
    let h: i64 = get();
    let w: i64 = get();
    let k: i64 = get();
    let hpe = get_divs_and_phis(h);
    let wpe = get_divs_and_phis(w);
    let mut ans = 0;
    for &(ha, phi1) in &hpe {
        for &(wb, phi2) in &wpe {
            let g = gcd(ha, wb);
            let mut tmp = powmod(k, h * w / ha / wb * g);
            tmp = tmp * phi1 % MOD;
            tmp = tmp * phi2 % MOD;
            ans = (ans + tmp) % MOD;
        }
    }
    ans = ans * powmod(h * w % MOD, MOD - 2) % MOD;
    println!("{}", ans);
}
