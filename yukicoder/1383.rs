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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn nth(a: i128, n: i64) -> i128 {
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((128 + n - 1) / n)) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i128;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn calc(x: i64, k: i64) -> (i64, bool) {
    let k = k as i128;
    let val = 4 * x as i128 + k * k;
    let y = nth(val, 2);
    ((y - k) as i64 / 2, y * y == val)
}

// https://yukicoder.me/problems/no/1383 (3.5)
// 20! ~= 2 * 10^18 > 10^18 であるため、B としては 1 <= B <= 18 の範囲まで考えれば良い。
// A についても B についても A(A + K)...(A+BK) は単調増加であるため、1 <= X <= N のとき f(X) <= 18 である。
// B >= 2 であれば A は 10^6 + 10^4 + .. 通り以下しかないので全列挙できる。B >= 2 について全列挙して、それが A(A + K) の形で表せるかどうかを調べれば良さそう。表せるのであればそれに対する f(X) の値が 1 増え、その後ここで見られなかった X = A(A + K) の形の整数全てに対して f(X) = 1 ということにすれば、f(X) = M である X の個数を数えることができる。
// -> K が 10^18 までということを見落とし、実装すべき式を間違えて WA。
fn main() {
    let n: i64 = get();
    let k: i64 = get();
    let m: i64 = get();
    let mut hm = HashMap::new();
    for b in 2i64..19 {
        let mut x = 1i64;
        loop {
            let mut prod = 1i64;
            for i in 0..b + 1 {
                prod = prod.saturating_mul(x.saturating_add(i.saturating_mul(k)));
            }
            if prod > n {
                break;
            }
            *hm.entry(prod).or_insert(0) += 1;
            x += 1;
        }
    }
    let mut ans = 0;
    let mut has = 0;
    for (&val, v) in hm.iter_mut() {
        if calc(val, k).1 {
            *v += 1;
            has += 1;
        }
        if *v == m {
            ans += 1;
        }
    }
    if m == 1 {
        let y = calc(n, k).0 - has;
        ans += y;
    }
    println!("{}", ans);
}
