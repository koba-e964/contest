use std::collections::*;
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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

const MOD: i64 =  998_244_353;

fn rec(
    n: i64, m: i64, carry: bool, x: usize,
    memo: &mut HashMap<(i64, i64, bool, usize), i64>,
) -> i64 {
    if n <= 0 || m <= 0 {
        return 0;
    }
    if n == 1 && m == 1 {
        return if x == 0 { 1 } else { 0 };
    }
    let key = (n, m, carry, x);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut tot = 0;
    for a in 0..10 {
        if a >= n { continue; }
        for b in 0..10 {
            if b >= m { continue; }
            let sum = a + b + if carry { 1 } else { 0 };
            if sum >= 10 {
                if x >= 1 {
                    tot += rec((n - a + 9) / 10, (m - b + 9) / 10, true, x - 1, memo);
                }
            } else {
                tot += rec((n - a + 9) / 10, (m - b + 9) / 10, false, x, memo);
            }
        }
    }
    tot %= MOD;
    memo.insert(key, tot);
    tot
}

// https://yukicoder.me/problems/no/2302 (3.5)
// 桁 DP。
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: i64 = get();
        let x: usize = get();
        let mut memo = HashMap::new();
        println!("{}", rec(n + 1, n + 1, false, x, &mut memo));
    }
}
