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
        n: usize, t: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![0usize; n];
    for (a, b) in ab {
        g[a] |= 1 << b;
        g[b] |= 1 << a;
    }
    let mut indep = vec![false; 1 << n];
    for bits in 1..1 << n {
        indep[bits] = (0..n).all(|i| (bits & 1 << i) == 0 || (g[i] & bits) == 0);
    }
    let mut dp = vec![vec![0u64; 1 << n]; t + 1];
    dp[0][0] = 1;
    for i in 1..t + 1 {
        for bits in 0..1 << n {
            let mut me = 0;
            for sub in subsets(bits) {
                if bits <= 2 * sub && indep[sub] {
                    me += dp[i - 1][bits - sub];
                }
            }
            dp[i][bits] = me;
        }
    }
    println!("{}", dp[t][(1 << n) - 1]);
}
