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

fn calc_step(s: &[char], k: usize, c: usize) -> Vec<usize> {
    assert_eq!(s[0], 'x');
    let n = s.len();
    const B: usize = 18;
    let mut dp = vec![vec![n; n + 1]; B];
    let mut nxt = vec![n; n + 1];
    for i in (0..n).rev() {
        if s[i] == 'o' {
            nxt[i] = i;
        } else {
            nxt[i] = nxt[i + 1];
        }
    }
    for i in 1..n {
        if i + c + 1 < n {
            dp[0][i] = nxt[i + c + 1];
        }
    }
    dp[0][0] = nxt[0];
    for i in 1..B {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }
    let mut ans = vec![];
    for i in 0..n {
        if s[i] == 'x' { continue; }
        // Is i skippable?
        let mut tot = 0;
        let mut cur = 0;
        for j in (0..B).rev() {
            let seeing = dp[j][cur];
            let to = if seeing == i {
                nxt[seeing + 1]
            } else {
                seeing
            };
            if to < i {
                tot += 1 << j;
                cur = to;
            }
        }
        let seeing = dp[0][cur];
        let mut done = false;
        if seeing == n {
            done = true;
        }
        else if seeing == i {
            cur = nxt[seeing + 1];
            if cur < n {
                tot += 1;
            } else {
                done = true;
            }
        } else {
            cur = seeing;
            tot += 1;
        }
        if !done {
            assert!(cur > i, "{} > {}", cur, i);
            for j in (0..B).rev() {
                if cur >= n { break; }
                let to = dp[j][cur];
                if to < n {
                    tot += 1 << j;
                    cur = to;
                }
            }
        }
        if tot < k {
            ans.push(i);
        }
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        _n: usize, k: usize, c: usize,
        s: chars,
    }
    let mut s = s;
    s.insert(0, 'x'); // sentinel
    let ans = calc_step(&s, k, c);
    for v in ans {
        puts!("{}\n", v);
    }
}
