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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/*
 * Z algorithm. Calculates an array a[i] = |lcp(s, s[i...|s|])|,
 * where s is the given string.
 * If n = s.length(), the returned array has length n + 1.
 * E.g. z_algorithm("ababa") = {5, 0, 3, 0, 1, 0}
 * Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
 * Verified by: AtCoder ARC055-C (http://arc055.contest.atcoder.jp/submissions/1061771)
 */
fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    ret[0] = n;
    let mut i = 1; let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] { j += 1; }
        ret[i] = j;
        if j == 0 { i += 1; continue; }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k; j -= k;
    }
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(a: chars);
    let z = z_algorithm(&a);
    let n = a.len();
    for i in 0..n {
        puts!("{}{}", z[i], if i + 1 == n { "\n" } else { " " });
    }
}
