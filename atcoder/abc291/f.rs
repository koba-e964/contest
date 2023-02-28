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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        s: [chars; n],
    }
    const INF: i32 = 1 << 28;
    let mut left = vec![INF; n];
    left[0] = 0;
    for i in 1..n {
        for j in 1..min(i, m) + 1 {
            if s[i - j][j - 1] == '1' {
                left[i] = min(left[i], left[i - j] + 1);
            }
        }
    }
    let mut right = vec![INF; n];
    right[n - 1] = 0;
    for i in (0..n - 1).rev() {
        for j in 1..min(n - i - 1, m) + 1 {
            if s[i][j - 1] == '1' {
                right[i] = min(right[i], right[i + j] + 1);
            }
        }
    }
    let mut res = vec![0; n - 2];
    for i in 1..n - 1 {
        let mut mi = INF;
        for j in max(i, m) - m..i {
            for k in 1..m + 1 {
                if j + k <= i || s[j][k - 1] != '1' { continue; }
                mi = min(mi, left[j] + right[j + k] + 1);
            }
        }
        res[i - 1] = if mi >= INF { -1 } else { mi };
    }
    putvec!(res);
}
