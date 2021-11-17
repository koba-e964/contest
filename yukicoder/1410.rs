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

const MOD: i64 = 1_000_000_007;

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


// https://yukicoder.me/problems/no/1410 (3)
// P S_{15-i} Q = 2^N - 1 - P S_i Q。
// R は P S_i Q と2^N - 1 - P S_i Q の間でなければならない (両端含む)。
// P S_i Q < 2^{N-1} のとき、このような R は 2^N - 2 * P S_i Q 個ある。
// P S_i Q >= 2^{N-1} のとき、このような R は 2 * P S_i Q + 2 - 2^N 個ある。
// これらの値から 2 を引いたものを考える。
// P S_i Q = 1 となる 0 <= P, Q <= 1 が x 個の場合 (1 <= x <= 4)、LSB 以外のビットが 1 になる (P, Q) は (x * x + (4 - x) * (4 - x)) * 2^{2(N - 2)} 通り。これらの和をとると (x * x + (4 - x)) * (4 - x) * 2^{2(N - 2)} * (2^N - 2) 通り。これに 2^{2N + 1} を足したものが答え。
fn main() {
    let n: i64 = get();
    let k: Vec<_> = get_word().chars().collect();
    let mut tot = 0;
    for i in 0..8 {
        if k[i] == 'o' {
            if n == 0 {
                tot += 1;
                continue;
            }
            let x = i.count_ones() as i64;
            let mut tmp = x * x + (4 - x) * (4 - x);
            tmp = tmp * powmod(2, 2 * n - 3 + MOD - 1) % MOD;
            tmp = tmp * (powmod(2, n - 1) - 1) % MOD;
            tot = (tot + tmp + powmod(2, 2 * n + 1)) % MOD;
        }
    }
    println!("{}", tot);
}
