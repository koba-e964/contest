use std::collections::*;
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

fn rec(unused: usize, a: &[i64], tot: i64, seen: &mut HashSet<i64>) {
    let n = a.len();
    if unused == 0 {
        seen.insert(tot);
        return;
    }
    let lowest = unused & unused.wrapping_neg();
    for bits in subsets(unused) {
        if (bits & unused) != bits || (bits & lowest) == 0 {
            continue;
        }
        let mut now = 0;
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                now += a[i];
            }
        }
        rec(unused - bits, a, tot ^ now, seen);
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut seen = HashSet::new();
    rec((1 << n) - 1, &a, 0, &mut seen);
    println!("{}", seen.len());
}
