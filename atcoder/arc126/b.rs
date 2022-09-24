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

// ret[i] = max {|L| : L is an increasing subsequence that ends at a[i] w.r.t cmp}
// If cmp = i64::lt, this function finds (usual) increasing subsequences.
// If cmp = i64::le, this function finds non-decreasing subsequences.
// Complexity: O(n log n)
// Verified by: https://atcoder.jp/contests/past202112-open/submissions/28433411
fn lis_by<F: FnMut(&i64, &i64) -> bool>(a: &[i64], mut cmp: F) -> Vec<usize> {
    let n = a.len();
    const INF: i64 = 1 << 60; // change here
    let mut dp = vec![INF; n + 1];
    let mut ans = vec![0; n];
    dp[0] = -INF;
    for i in 0..n {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if cmp(&dp[mid], &a[i]) {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        ans[i] = pass + 1;
        dp[pass + 1] = std::cmp::min(dp[pass + 1], a[i]);
    }
    ans
}

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut ab = ab;
    ab.sort_by_key(|&(a, b)| (a, n - b));
    let mut tmp = vec![0; m];
    for i in 0..m {
        tmp[i] = ab[i].1 as i64;
    }
    println!("{}", lis_by(&tmp, i64::lt).iter().max().unwrap());
}
