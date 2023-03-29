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

const MOD: i64 = 998_244_353;

fn rec(h: [i32; 4], w: [i32; 4], memo: &mut HashMap<([i32; 4], [i32; 4]), i64>) -> i64 {
    let key = (h, w);
    if h.iter().any(|&v| v < 0) || w.iter().any(|&v| v < 0) {
        return 0;
    }
    if h.iter().all(|&v| v == 0) || w.iter().all(|&v| v == 0) {
        return 1;
    }
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut ans = 0i64;
    for bits in 0..1 << 9 {
        let mut d = [[0; 4]; 4];
        for i in 0..9 {
            d[i / 3][i % 3] = (bits >> i) & 1;
        }
        for i in 0..3 {
            let mut tmp = h[i] % 2;
            for j in 0..3 {
                tmp ^= d[i][j];
            }
            d[i][3] = tmp;
        }
        for i in 0..4 {
            let mut tmp = w[i] % 2;
            for j in 0..3 {
                tmp ^= d[j][i];
            }
            d[3][i] = tmp;
        }
        let mut r = [0; 4];
        let mut c = [0; 4];
        for i in 0..4 {
            for j in 0..4 {
                r[i] += d[i][j];
                c[j] += d[i][j];
            }
        }
        if (0..4).any(|i| h[i] < r[i] || (h[i] + r[i]) % 2 != 0)
            || (0..4).any(|i| w[i] < c[i] || (w[i] + c[i]) % 2 != 0) {
            continue;
        }
        for i in 0..4 {
            r[i] = (h[i] - r[i]) >> 1;
            c[i] = (w[i] - c[i]) >> 1;
        }
        ans += rec(r, c, memo);
        if ans >= MOD {
            ans -= MOD;
        }
    }
    memo.insert(key, ans);
    ans
}

// https://yukicoder.me/problems/no/2159 (4)
// Solved with hints
// 下位ビットから決定していけば良い。2^16 * 30 * 2^8 回くらいの計算でできる。
// HashMap あたりでメモ化すればよさそう。
// 2^16 のパートは、最下位ビットを 9 箇所定めれば残りが自動的に決まることから、2^9 にできる。この実装では 2^12 にした。
// -> TLE したので 2^9 にした。
fn main() {
    let mut h = [0i32; 4];
    let mut w = [0i32; 4];
    for i in 0..4 {
        h[i] = get::<i32>() - 4;
    }
    for i in 0..4 {
        w[i] = get::<i32>() - 4;
    }
    let mut memo = HashMap::new();
    println!("{}", rec(h, w, &mut memo));
}
