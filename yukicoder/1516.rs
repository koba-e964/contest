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

fn squmul(a: &[Vec<i64>], b: &[Vec<i64>], mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
                ret[i][k] %= mo;
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<i64>], mut e: i64, mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut sum = vec![vec![0; n]; n];
    for i in 0..n { sum[i][i] = 1; }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur, mo);
        }
        cur = squmul(&cur, &cur, mo);
        e /= 2;
    }
    sum
}

const MOD: i64 = 998_244_353;

// https://yukicoder.me/problems/no/1513 (2.5)
// https://yukicoder.me/problems/no/1516 (3.5)
// O(K^6 log N) or O(K^2 N) (累積和で加速)。
// dpgt[i+1][a][b] += dplt[i][b][c] (a > b, c != a)
// dplt[i+1][a][b] += dpgt[i][b][c] (a < b, c != a)
// epgt[i+1][a][b] += eplt[i][b][c] + a * dplt[b][c] (a > b, c != a)
// eplt[i+1][a][b] += epgt[i][b][c] + a * dpgt[b][c] (a < b, c != a)
fn main() {
    let n: i64 = get();
    let k: usize = get();
    let mut mat = vec![vec![0; 2 * k * k]; 2 * k * k];
    for a in 0..k {
        for b in 0..k {
            for c in 0..k {
                if c == a { continue; }
                if (a > b && b < c) || (a < b && b > c) {
                    mat[b * k + c][a * k + b] += 1;
                    mat[k * k + b * k + c][k * k + a * k + b] += 1;
                    mat[b * k + c][k * k + a * k + b] += a as i64;
                }
            }
        }
    }
    let pw = squpow(&mat, n - 2, MOD);
    let mut ans1 = 0;
    let mut ans2 = 0;
    for a in 0..k {
        for b in 0..k {
            if a != b {
                for c in 0..k {
                    for d in 0..k {
                        if c != d {
                            ans1 = (ans1 + pw[a * k + b][c * k + d]) % MOD;
                            ans2 = (ans2 + pw[a * k + b][k * k + c * k + d]) % MOD;
                            ans2 = (ans2 + pw[a * k + b][c * k + d] * (a + b) as i64) % MOD;
                        }
                    }
                }
            }
        }
    }
    println!("{} {}", ans1, ans2);
}
