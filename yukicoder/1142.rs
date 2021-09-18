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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 1_000_000_007;

fn calc(a: &[usize]) -> Vec<i64> {
    let n = a.len();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] ^ a[i];
    }
    let mut dp = vec![0i64; 1024];
    for &a in &acc {
        dp[a] += 1;
    }
    for i in 0..10 {
        for bits in 0..1024 {
            if (bits & 1 << i) == 0 {
                let x = dp[bits];
                let y = dp[bits | 1 << i];
                dp[bits] = x + y;
                dp[bits | 1 << i] = x - y;
            }
        }
    }
    for bits in 0..1024 {
        dp[bits] = dp[bits] * dp[bits];
    }
    for i in 0..10 {
        for bits in 0..1024 {
            if (bits & 1 << i) == 0 {
                let x = dp[bits];
                let y = dp[bits | 1 << i];
                dp[bits] = (x + y) / 2;
                dp[bits | 1 << i] = (x - y) / 2;
            }
        }
    }
    dp[0] -= (n + 1) as i64;
    for bits in 0..1024 {
        dp[bits] /= 2;
        dp[bits] %= MOD;
    }
    eprintln!("{:?}", &dp[..16]);
    dp
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let c = calc(&a);
    let d = calc(&b);
    let mut ans = 0;
    for i in 0..1024 {
        ans = (ans + c[i] * d[k ^ i]) % MOD;
    }
    println!("{}", ans);
}
