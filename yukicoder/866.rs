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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: log-factor-optimization
fn main() {
    input! {
        h: usize, w: usize, k: usize,
        c: [chars; h],
    }
    let mut acc = vec![vec![[0; 26]; w + 1]; h + 1];
    for u in 0..26 {
        for i in 0..h {
            for j in 0..w {
                acc[i + 1][j + 1][u] = acc[i + 1][j][u] + acc[i][j + 1][u]
                    - acc[i][j][u] + if c[i][j] as u8 - b'a' == u as u8 {
                        1
                    } else {
                        0
                    };
            }
        }
    }
    let kind = |i: usize, j: usize, x: usize| -> usize {
        let mut ans = 0;
        for u in 0..26 {
            if acc[i + x][j + x][u] - acc[i][j + x][u]
                - acc[i + x][j][u] + acc[i][j][u] > 0 {
                    ans += 1;
                }
        }
        ans
    };
    let mut tot = 0;
    for (is, js) in (0..h).map(|i| (i, 0)).chain((1..w).map(|j| (0, j))) {
        let mut lb = 0;
        let mut ub = 0;
        let lim = min(h - is, w - js);
        for i in 0..lim {
            lb = max(lb, i);
            ub = max(ub, i);
            while lb <= lim && kind(is + i, js + i, lb - i) < k {
                lb += 1;
            }
            while ub <= lim && kind(is + i, js + i, ub - i) <= k {
                ub += 1;
            }
            tot += (ub - lb) as i64;
        }
    }
    println!("{}", tot);
}
