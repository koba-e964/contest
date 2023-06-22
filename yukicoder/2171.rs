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

// Sparse Table.
// BiOp should be the type of a binary operator which is
// associative, commutative and idempotent.
// (For example, both min and gcd satisfy these properties.)
// Verified by: yukicoder No. 2171
// (https://yukicoder.me/submissions/883410)
struct SparseTable<T, BiOp> {
    biop: BiOp,
    st: Vec<Vec<T>>,
}

impl<T, BiOp> SparseTable<T, BiOp>
    where BiOp: Fn(T, T) -> T,
          T: Copy {
    pub fn new(ary: &[T], biop: BiOp) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i];
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = biop(st[b - 1][i], st[b - 1][next_idx]);
            }
        }
        SparseTable {biop: biop, st: st}
    }
    fn top_bit(t: usize) -> usize {
        8 * std::mem::size_of::<usize>() - 1 - t.leading_zeros() as usize
    }
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let (f, s) = (range.start, range.end - 1);
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f], self.st[b][endpoint])
    }
}

// https://yukicoder.me/problems/no/2171 (3)
// 各 i に対して最終的な A_i の値として考えられるのは or(A_j, ..., A_i) の形の値なので 30 通り程度。それぞれに対してどこを左端とするかをあらかじめ計算しておく。
// 特定のパターンがあり得ることと、A_i の値が or(A_j, ..., A_i) であるような最大の j を L_i としたとき L_i <= L_{i+1} が成立することが同値である。よって DP でできる。
// W = 30 として O(NW^2)-time である。
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut pts = vec![vec![]; n];
    let mut last = vec![];
    let spt = SparseTable::new(&a, |a, b| a | b);
    for i in 0..n {
        last.push(i);
        for val in &mut last {
            // right-shift val as much as possible
            while *val < i && spt.query(*val..i + 1) == spt.query(*val + 1..i + 1) {
                *val += 1;
            }
        }
        last.dedup();
        pts[i] = last.clone();
    }
    const MOD: i64 = 998_244_353;
    let mut dp = vec![1];
    for i in 1..n {
        let mut ep = vec![0; pts[i].len()];
        for j in 0..pts[i - 1].len() {
            for k in 0..pts[i].len() {
                if pts[i - 1][j] <= pts[i][k] {
                    ep[k] += dp[j];
                    if ep[k] >= MOD {
                        ep[k] -= MOD;
                    }
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp.iter().sum::<i64>() % MOD);
}
