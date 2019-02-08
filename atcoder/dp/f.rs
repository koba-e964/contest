#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        s: chars,
        t: chars,
    }
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0 .. n {
        for j in 0 .. m + 1 {
            dp[i + 1][j] = dp[i][j];
            if j > 0 {
                let u = if s[i] == t[j - 1] { 1 } else { 0 };
                dp[i + 1][j] = max(dp[i + 1][j],
                                   max(dp[i][j - 1] + u, dp[i + 1][j - 1]));
            }
        }
    }
    let mut ans = Vec::new();
    let (mut x, mut y, mut cur) = (n, m, dp[n][m]);
    while cur > 0 {
        assert_eq!(cur, dp[x][y]);
        if dp[x - 1][y - 1] + 1 == dp[x][y] && s[x - 1] == t[y - 1] {
            ans.push(s[x - 1]);
            x -= 1;
            y -= 1;
            cur -= 1;
            continue;
        }
        if dp[x - 1][y] == dp[x][y] {
            x -= 1;
            continue;
        }
        if dp[x][y - 1] == dp[x][y] {
            y -= 1;
            continue;
        }
        x -= 1;
        y -= 1;
    }
    ans.reverse();
    puts!("{}\n", ans.into_iter().collect::<String>());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
