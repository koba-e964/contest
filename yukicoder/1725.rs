use std::cmp::*;
use std::io::{Write, BufWriter};
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

fn dfs(k: usize, l: i64, r: i64, c: usize,
       len: &[i64], x: &[[i64; 26]], y: &[[i64; 26]]) -> i64 {
    if l == r {
        return 0;
    }
    if k == 1 {
        return x[r as usize][c] - x[l as usize][c];
    }
    assert!(r <= len[k]);
    assert!(l <= r);
    if (l, r) == (0, len[k]) {
        let mut a = y[y.len() - 1][c];
        let b = dfs(k - 1, 0, len[k - 1], c, len, x, y);
        a += 2 * b;
        return a;
    }
    let mut a = 0;
    let mid1 = len[k - 1];
    let mid2 = len[k] - len[k - 1];
    if l < mid1 {
        let b = dfs(k - 1, l, min(r, mid1), c, &len, x, y);
        a += b;
    }
    if l < mid2 && r > mid1 {
        let lidx = (max(l, mid1) - mid1) as usize;
        let ridx = (min(r, mid2) - mid1) as usize;
        a += y[ridx][c] - y[lidx][c];
    }
    if r > mid2 {
        let b = dfs(k - 1,
                    len[k - 1] - (r - mid2),
                    len[k - 1] - (max(l, mid2) - mid2), c, &len, x, y);
        a += b;
    }
    a
}

// Tags: exploit-small-constraints
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        x: chars,
        y: chars,
        q: usize,
        lrc: [(i64, i64, char); q],
    }
    fn prepare(x: &[char]) -> Vec<[i64; 26]> {
        let n = x.len();
        let mut dp = vec![[0; 26]; n + 1];
        for i in 0..n {
            dp[i + 1] = dp[i];
            dp[i + 1][(x[i] as u8 - b'a') as usize] += 1;
        }
        dp
    }
    let dpx = prepare(&x);
    let dpy = prepare(&y);
    let mut len = vec![0, x.len() as i64];
    let mut cur = x.len() as i64;
    let mut idx = 1;
    while cur < 1i64 << 30 {
        cur = 2 * cur + y.len() as i64;
        idx += 1;
        len.push(cur);
    }
    for (l, r, c) in lrc {
        puts!("{}\n", dfs(idx, l - 1, r, (c as u8 - b'a') as _, &len, &dpx, &dpy));
    }
}
