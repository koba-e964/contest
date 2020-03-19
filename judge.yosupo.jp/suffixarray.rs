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
 * Suffix Array by Manber & Myers.
 * Verified by: AtCoder ARC050 (http://arc050.contest.atcoder.jp/submissions/818912)
 * Reference: http://mayokoex.hatenablog.com/entry/2016/04/03/145845
 */
fn create_sa<T: Ord + Clone>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0 .. n + 1).collect();
    let mut rank: Vec<usize> = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    let mut coord = s.to_vec();
    coord.sort();
    coord.dedup();

    for i in 0..n + 1 {
        rank[i] = if i < n { coord.binary_search(&s[i]).unwrap() + 1 } else { 0_usize };
    }
    let mut k = 1;
    while k <= n {
        {
            let key = |i: &usize| {
                let ri = if i + k <= n { rank[i + k] as i32 } else { -1 };
                (rank[*i], ri)
            };
            sa.sort_by_key(&key);
            tmp[sa[0]] = 0;
            for i in 1 .. n + 1 {
                tmp[sa[i]] = tmp[sa[i - 1]]
                    + if key(&sa[i - 1]) < key(&sa[i]) { 1 } else { 0 };
            }
        }
        rank.clone_from_slice(&tmp);
        k *= 2;
    }
    return sa;
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(a: chars);
    let sa = create_sa(&a);
    let n = a.len();
    for i in 1..n + 1 {
        puts!("{}{}", sa[i], if i == n { "\n" } else { " " });
    }
}
