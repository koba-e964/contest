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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

fn main() {
    input! {
        t: i64,
        n: usize,
        a: [i64; n],
    }
    let mut ok = vec![false; 1 << n];
    for bits in 1..1 << n {
        let mut s = 0;
        for i in 0..n {
            if (bits & 1 << i) != 0 { s += a[i]; }
        }
        ok[bits] = s <= t;
    }
    let mut dp = vec![0; 1 << n];
    for bits in 1usize..1 << n {
        let mut me = 100;
        let lsb = bits & bits.wrapping_neg();
        for sub in subsets(bits ^ lsb) {
            if ok[sub | lsb] {
                me = min(me, dp[bits ^ sub ^ lsb] + 1);
            }
        }
        dp[bits] = me;
    }
    println!("{}", dp[(1 << n) - 1]);
}
