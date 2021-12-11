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

const MOD: i64 = 998_244_353;

// https://yukicoder.me/problems/no/1513 (2.5)
// https://yukicoder.me/problems/no/1516 (3.5)
// O(K^6 log N) or O(K^2 N) (累積和で加速)。
// dpgt[i+1][a][b] += dplt[i][b][c] (a > b, c != a)
// dplt[i+1][a][b] += dpgt[i][b][c] (a < b, c != a)
// epgt[i+1][a][b] += eplt[i][b][c] + a * dplt[b][c] (a > b, c != a)
// eplt[i+1][a][b] += epgt[i][b][c] + a * dpgt[b][c] (a < b, c != a)
fn main() {
    let n: usize = get();
    let k: usize = get();
    let mut dp = vec![vec![0; k]; k];
    let mut dpacc = vec![vec![0; k + 1]; k];
    let mut ep = vec![vec![0; k]; k];
    let mut epacc = vec![vec![0; k + 1]; k];
    for a in 0..k {
        for b in 0..k {
            if a != b {
                dp[a][b] += 1;
                ep[a][b] += (a + b) as i64;
            }
        }
    }
    for a in 0..k {
        for b in 0..k {
            dpacc[a][b + 1] = dpacc[a][b] + dp[a][b];
            epacc[a][b + 1] = epacc[a][b] + ep[a][b];
        }
    }
    for _ in 0..n - 2 {
        let mut ndp = vec![vec![0; k]; k];
        let mut nep = vec![vec![0; k]; k];
        for a in 0..k {
            for b in 0..k {
                if a == b { continue; }
                if a < b {
                    ndp[a][b] = (ndp[a][b] + dpacc[b][b] - dp[b][a] + MOD) % MOD;
                    nep[a][b] = (nep[a][b] + epacc[b][b] - ep[b][a] + MOD) % MOD;
                    nep[a][b] = (nep[a][b] + (dpacc[b][b] - dp[b][a] + MOD) * a as i64) % MOD;
                } else {
                    ndp[a][b] = (ndp[a][b] + dpacc[b][k] - dpacc[b][b + 1] - dp[b][a] + 2 * MOD) % MOD;
                    nep[a][b] = (nep[a][b] + epacc[b][k] - epacc[b][b + 1] - ep[b][a] + 2 * MOD) % MOD;
                    nep[a][b] = (nep[a][b] + (dpacc[b][k] - dpacc[b][b + 1] - dp[b][a] + 2 * MOD) * a as i64) % MOD;
                }
            }
        }
        dp = ndp;
        ep = nep;
        for a in 0..k {
            for b in 0..k {
                dpacc[a][b + 1] = dpacc[a][b] + dp[a][b];
                epacc[a][b + 1] = epacc[a][b] + ep[a][b];
            }
        }
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for c in 0..k {
        for d in 0..k {
            if c != d {
                ans1 = (ans1 + dp[c][d]) % MOD;
                ans2 = (ans2 + ep[c][d]) % MOD;
            }
        }
    }
    println!("{} {}", ans1, ans2);
}
