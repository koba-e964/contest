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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

// https://yukicoder.me/problems/no/2025 (3.5)
// dp[i][j] := i 以降で j 文字取る方法の総数
// とすると、この DP 配列に沿って貪欲できる。
// (例: 1 が多量にある場合、dp[2][0] + ... dp[2][c] < L である最大の c について、1 は L-c-1 個取るべきである。)
// dp[i][..j] の累積和を計算しておけば、この貪欲は O(log L)-time で実行できる。
// 細かい添え字の調整は https://qiita.com/kobae964/private/0290fdd0b5ffdcd7dd5b の方法で。
// Tags: off-by-one-errors, dp
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
        n: usize, l: usize,
        c: [usize; n],
        q: usize,
        k: [i128; q],
    }
    let mut dp = vec![vec![0i128; l + 1]; n + 1];
    let mut acc = vec![vec![0i128; l + 2]; n + 1];
    dp[n][0] = 1;
    for i in 1..l + 2 {
        acc[n][i] = 1;
    }
    for i in (0..n).rev() {
        for j in 0..l + 1 {
            let tmp = acc[i + 1][j + 1].wrapping_sub(acc[i + 1][max(j, c[i]) - c[i]]);
            dp[i][j] = tmp;
            acc[i][j + 1] = acc[i][j].wrapping_add(tmp);
        }
    }
    for mut k in k {
        k -= 1;
        if k >= dp[0][l] {
            puts!("-1\n");
            continue;
        }
        let mut rem = l;
        let mut ans = vec![0; n];
        for i in 0..n {
            let lim = min(rem, c[i]);
            let margin = rem - lim;
            let idx = acc[i + 1].upper_bound(&(k + acc[i + 1][margin]));
            let now = idx - margin - 1;
            ans[i] = lim - now;
            rem -= lim - now;
            k -= acc[i + 1][now + margin] - acc[i + 1][margin];
        }
        assert_eq!(k, 0);
        putvec!(ans);
    }
}
