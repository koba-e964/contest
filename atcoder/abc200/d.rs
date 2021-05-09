#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

const B: usize = 200;
fn recov(dp: &[Vec<i64>], a: &[usize], mut cur: usize, mut cur2: usize) -> Vec<usize> {
    let mut path = vec![];
    while dp[cur][cur2] > 1 {
        if cur > 0 && dp[cur - 1][cur2] == dp[cur][cur2] {
            cur -= 1;
            continue;
        }
        assert_eq!(dp[cur - 1][(cur2 + B - a[cur - 1]) % B] + 1, dp[cur][cur2]);
        path.push(cur);
        cur -= 1;
        cur2 = (cur2 + B - a[cur]) % B;
    }
    loop {
        if cur2 == a[cur - 1] {
            path.push(cur);
            break;
        }
        cur -= 1;
    }
    path.sort();
    path
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [usize; n],
    }
    const INF: i64 = 1 << 30;
    let mut dp = vec![vec![INF; B]; n + 1];
    let a: Vec<usize> = a.into_iter().map(|x| x % B).collect();
    for i in 0..n {
        for j in 0..B {
            let mut me = dp[i][j];
            me.chmin(dp[i][(j + B - a[i]) % B] + 1);
            dp[i + 1][j] = me;
        }
        dp[i + 1][a[i]].chmin(1);
    }
    for i in 0..n {
        for j in 0..B {
            if dp[i][j] < INF && dp[i][(j + B - a[i]) % B] < INF {
                let x = recov(&dp, &a, i, j);
                let mut y = recov(&dp, &a, i, (j + B - a[i]) % B);
                y.push(i + 1);
                puts!("Yes\n");
                puts!("{} ", x.len());
                putvec!(x);
                puts!("{} ", y.len());
                putvec!(y);
                return;
            }
        }
    }
    for i in 0..n {
        if dp[i][a[i]] < INF {
            let x = recov(&dp, &a, i, a[i]);
            let mut y = vec![];
            y.push(i + 1);
            puts!("Yes\n");
            puts!("{} ", x.len());
            putvec!(x);
            puts!("{} ", y.len());
            putvec!(y);
            return;
        }
    }
    for i in 1..n {
        if dp[i][0] < INF {
            let mut x = recov(&dp, &a, i, 0);
            let y = vec![i + 1];
            x.push(i + 1);
            puts!("Yes\n");
            puts!("{} ", x.len());
            putvec!(x);
            puts!("{} ", y.len());
            putvec!(y);
            return;
        }
    }
    puts!("No\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
