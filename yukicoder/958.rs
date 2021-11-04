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

// Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
// Verified by https://atcoder.jp/contests/wupc2019/submissions/4540033
fn manacher<T: PartialEq>(tmp: &[T]) -> Vec<usize> {
    let n = tmp.len();
    let mut r = vec![0; n];
    {
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while i >= j && i + j < n && tmp[i - j] == tmp[i + j] {
                j += 1;
            }
            r[i] = j;
            let mut k = 1;
            while i >= k && i + k < n && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
    }
    r
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: i64, q: usize,
        s: chars,
        a: [i64; q],
    }
    let mut t = s.clone();
    t.extend(&s);
    t.extend(&s);
    let dp = manacher(&t);
    for a in a {
        let a = a - 1;
        let idx = (a as usize % n) + n;
        let len = dp[idx] - 1;
        let lim = min(a, n as i64 * k - 1 - a);
        if idx <= len || idx + len + 1 >= 3 * n {
            puts!("{}\n", lim * 2 + 1);
            continue;
        } else {
            puts!("{}\n", 2 * min(lim, len as i64) + 1);
        }
    }
}
