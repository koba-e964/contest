use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn lis(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    const INF: i64 = 1 << 60;
    let mut dp = vec![INF; n + 1];
    let mut ans = vec![0; n];
    dp[0] = -INF;
    for i in 0..n {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if dp[mid] <= a[i] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        ans[i] = pass + 1;
        dp[pass + 1] = min(dp[pass + 1], a[i]);
    }
    ans
}

const MOD: i64 = 998_244_353;

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}
// https://yukicoder.me/problems/no/2230 (3)
// 経路内部での良い知らせが言い伝えられているマスを最大化することになる。x でソートした後 LIS を求めれば良い。
// Tags: longest-increasing-subsequences
fn main() {
    input! {
        h: i64, w: i64, n: usize, p: i64,
        xy: [(i64, i64); n],
    }
    let mut xy = xy;
    xy.sort();
    let mut ys = vec![0; n];
    for i in 0..n {
        ys[i] = xy[i].1;
    }
    let lis = lis(&ys);
    let &ma = lis.iter().max().unwrap_or(&0);
    let mut tmp = powmod(p, MOD - 1 - (h + w - 3), MOD);
    tmp = tmp * powmod(p - 2, ma as i64, MOD) % MOD;
    tmp = tmp * powmod(p - 1, h + w - 3 - ma as i64, MOD) % MOD;
    println!("{}", (MOD - tmp + 1) % MOD);
}
