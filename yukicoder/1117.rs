use std::collections::*;
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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

const INF: i64 = 1 << 50;
fn prog(n: usize, acc: &[i64], m: usize, dp: &[i64], ep: &mut [i64]) {
    let mut st = VecDeque::new();
    st.push_back((0, dp[0]));
    for i in 1..n + 1 {
        let pre = st[0];
        ep[i] = pre.1 + acc[i];
        if pre.0 + m <= i {
            st.pop_front();
        }
        while let Some(v) = st.pop_back() {
            if v.1 <= dp[i] - acc[i] {
                continue;
            }
            st.push_back(v);
            break;
        }
        st.push_back((i, dp[i] - acc[i]));
    }
    let mut st = VecDeque::new();
    st.push_back((0, dp[0]));
    for i in 1..n + 1 {
        let pre = st[0];
        ep[i].chmax(pre.1 - acc[i]);
        if pre.0 + m <= i {
            st.pop_front();
        }
        while let Some(v) = st.pop_back() {
            if v.1 <= dp[i] + acc[i] {
                continue;
            }
            st.push_back(v);
            break;
        }
        st.push_back((i, dp[i] + acc[i]));
    }
}

// Tags: sliding-window-technique
fn main() {
    // Lagrange relaxation seems not to work.
    input! {
        n: usize, k: usize, m: usize,
        a: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut dp = vec![-INF; n + 1];
    dp[0] = 0;
    for _ in 0..k {
        let mut ep = vec![-INF; n + 1];
        prog(n, &acc, m, &dp, &mut ep);
        dp = ep;
    }
    println!("{}", dp[n]);
}
