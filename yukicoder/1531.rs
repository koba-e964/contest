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

const MOD: i64 = 998_244_353;

fn dfs(n: i64, k: i64, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    if n == 0 {
        let p = k.count_ones();
        return (1i64 << p) % MOD;
    }
    let key = (n, k);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut ans = dfs(n / 2, k / 2, memo) * if k % 2 == 0 {
        1
    } else {
        2
    } % MOD;
    if n % 2 == 1 {
        ans += dfs(n / 2, (k + 1) / 2, memo) * if k % 2 == 1 {
            1
        } else {
            2
        };
        ans %= MOD;
    }
    memo.insert(key, ans);
    ans
}

// https://yukicoder.me/problems/no/1531 (4)
// (1+x)^K (1+x+y)^N を通常母関数として持つ 2 次元数列に含まれる奇数の個数が答え。これは (N and i) = i である i について (1+x)^{K+i} に含まれる奇数の個数 (つまり 2^popcount(K+i)) を足したものに等しい。
// Tags: fps
fn main() {
    let n: i64 = get();
    let k: i64 = get();
    let mut memo = HashMap::new();
    println!("{}", dfs(n, k, &mut memo));
}
