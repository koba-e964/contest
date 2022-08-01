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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

pub fn lastlen2(diff: i64) -> i64 {
    let mut l = 0;
    while diff > (1 << l) - 1 {
        l += 1;
    }
    if l == 0 {
        return 0;
    }
    if l <= 2 {
        return min(diff, 2);
    }
    if diff == (1 << l) - 1 {
        return 1 << (l - 1);
    }
    if diff >= 3 << (l - 2) {
        return diff - (1 << (l - 1)) + 2;
    }
    (1 << (l - 2)) + 1
}

// The author read the editorial before implementing this.
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut level = vec![0; n];
    let mut lastlen = vec![0; n];
    let mut lastlen2 = vec![0; n];
    for i in 1..n {
        let diff = a[i] - a[i - 1] - 1;
        let mut l = 0;
        while diff > (1 << l) - 1 {
            l += 1;
        }
        level[i] = l;
        lastlen[i] = if l == 0 { 0 } else { diff - (1 << (l - 1)) + 1 };
        lastlen2[i] = self::lastlen2(diff);
    }
    const B: usize = 61;
    let mut dp = vec![vec![0; n]; B];
    let mut acc = vec![vec![0; n]; B];
    dp[0][0] = 1;
    for i in 0..B {
        acc[i][0] = 1;
    }
    for j in 1..n {
        dp[0][j] = dp[0][j - 1] + 1;
        acc[0][j] = dp[0][j];
        for i in 1..B {
            let newdp = if i < level[j] {
                acc[i][j - 1] + (1 << (i - 1))
            } else {
                let mut tmp = acc[level[j]][j - 1] + lastlen[j];
                if level[j] > 0 {
                    tmp = max(tmp, acc[level[j] - 1][j - 1] + lastlen2[j]);
                }
                if i > level[j] {
                    tmp = max(tmp, acc[i][j - 1]);
                }
                tmp
            };
            dp[i][j] = max(dp[i][j], newdp);
            acc[i][j] = max(acc[i - 1][j], dp[i][j]);
        }
    }
    println!("{}", acc[B - 1].iter().max().unwrap());
}
