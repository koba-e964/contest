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

// n が奇数のとき、(n-1) * 25 * 26^{(n-1)/2}
// n >= 4 で n が偶数のとき、
// 中央の 2 文字を除いて回文: 26*25*26^{(n-2)/2}
// n/2-2, n/2 + 1 のどちらかを取り除いて回文: (2 *) 25*26*26^{(n-2)/2}
// n/2-2, n/2 + 1 のどちらを取り除いても回文: -26*25*26^{(n-4)/2} (包除原理なのでマイナス)
// それ以外の位置: ((n-4) *) (26*26-1)*26^{(n-2)/2}
// -> これだとサンプルが合わないので、サンプルから線形補完して最後のが (n-4) * (26*25)*26^{(n-2)/2} + 26 であることを発見した。これで AC。
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: i64 = get();
        if n % 2 == 1 {
            let tmp = powmod(26, (n - 1) / 2);
            println!("{}", tmp * ((n - 1) * 25) % MOD);
            continue;
        }
        let tmp = powmod(26, (n - 2) / 2);
        println!("{}", (tmp * (650 + 1275 + 650 * (n - 4) - 1) + 26) % MOD);
    }
}
