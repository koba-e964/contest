// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 100_000;

// Tags: low-memory, constant-factor-optimization
fn main() {
    input! {
        m: usize, n: usize,
        s: [chars; m],
    }
    let mut dp = vec![[0; 2]; 1 << (n - 1)];
    dp[0][0] = 1;
    let mut ep = vec![[0; 2]; 1 << (n - 1)];
    for i in 0..m * n {
        for bits in 0..1 << (n - 1) {
            ep[bits] = [0; 2];
        }
        let c = s[i / n][i % n];
        let idx = i % n;
        for bits in 0..1 << (n - 1) {
            let val = dp[bits];
            let off = bits & !(1 << idx);
            if c == 'J' || c == '?' {
                ep[off][1] += val[0] + val[1];
                ep[off][1] %= MOD;
            }
            if c == 'O' || c == '?' {
                ep[off][0] += val[0];
                ep[off][0] %= MOD;
                if idx > 0 {
                    ep[off | 1 << (idx - 1)][0] += val[1];
                    ep[off | 1 << (idx - 1)][0] %= MOD;
                }
            }
            if (c == 'I' || c == '?') && (bits & 1 << idx) == 0 {
                ep[off][0] += val[0] + val[1];
                ep[off][0] %= MOD;
            }
        }
        if (i + 1) % n == 0 {
            for bits in 0..1 << (n - 1) {
                let k = ep[bits];
                ep[bits] = [(k[0] + k[1]) % MOD, 0];
            }
        }
        std::mem::swap(&mut dp, &mut ep);
    }
    let mut tot = 0;
    for bits in 0..1 << (n - 1) {
        let k = dp[bits];
        tot = (tot + k[0] + k[1]) % MOD;
    }
    let mut p3 = 1;
    for i in 0..m {
        for j in 0..n {
            if s[i][j] == '?' {
                p3 = 3 * p3 % MOD;
            }
        }
    }
    println!("{}", (p3 - tot + MOD) % MOD);
}
