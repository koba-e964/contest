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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
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
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut indep = vec![false; 1 << n];
    let mut g = vec![0; n];
    for (u, v) in uv {
        g[u] |= 1 << v;
        g[v] |= 1 << u;
    }
    for bits in 0..1 << n {
        if (0..n).all(|i| (bits & 1 << i) == 0 || (g[i] & bits) == 0) {
            indep[bits] = true;
        }
    }
    let mut dp = vec![0; 1 << n];
    // O(3^n)
    const INF: i32 = 1 << 28;
    for bits in 1..1 << n {
        let mut me = INF;
        for sub in subsets(bits) {
            if sub == 0 || !indep[sub] { continue; }
            me = min(me, dp[bits - sub] + 1);
        }
        dp[bits] = me;
    }
    println!("{}", dp[(1 << n) - 1] - 1);
}
