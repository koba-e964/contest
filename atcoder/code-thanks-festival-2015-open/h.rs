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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize, k: i32,
        s: [chars; h],
    }
    let mut acc = vec![vec![0; w + 1]; h + 1];
    let mut cnt = vec![vec![[0; 10]; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            let idx = (s[i][j] as u8 - b'0') as usize;
            acc[i + 1][j + 1] = acc[i][j + 1] + acc[i + 1][j] - acc[i][j]
                + idx as i32;
            for k in 1..10 {
                cnt[i + 1][j + 1][k] = cnt[i + 1][j][k] + cnt[i][j + 1][k]
                    - cnt[i][j][k] + if k == idx { 1i32 } else { 0 };
            }
        }
    }
    let get = |(a, b): (usize, usize), (c, d): (usize, usize)|
    acc[c][d] - acc[c][b] - acc[a][d] + acc[a][b];
    let mut tot: i64 = 0;
    for i in 0..h {
        for j in i + 3..h + 1 {
            let mut pos = 0;
            for l in 0..w {
                while pos <= w {
                    let tot = get((i, l), (j, pos));
                    if tot >= k + 1 {
                        break;
                    }
                    pos += 1;
                }
                let mut pos2 = max(pos, l + 3);
                while pos2 <= w && get((i, l), (j, pos2)) <= k + 9 {
                    let diff = get((i, l), (j, pos2)) - k;
                    let diff = diff as usize;
                    let val =
                        (cnt[j - 1][pos2 - 1][diff] - cnt[i + 1][pos2 - 1][diff]
                         - cnt[j - 1][l + 1][diff] + cnt[i + 1][l + 1][diff]) as i64;
                    tot += val;
                    pos2 += 1;
                }
            }
        }
    }
    puts!("{}\n", tot);
}
