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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/2331 (3)
// 対角線を固定して、残り 2 点をその対角線から最も離れている点にすれば良い。
// 計算量は O(N^3)。
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ans = 0;
    for i in 0..n {
        let (a, b) = xy[i];
        for j in 0..i {
            let (c, d) = xy[j];
            let c = c - a;
            let d = d - b;
            const INF: i64 = 1 << 60;
            let mut mi = INF;
            let mut ma = -INF;
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                let (x, y) = xy[k];
                let x = x - a;
                let y = y - b;
                mi = min(mi, c * y - d * x);
                ma = max(ma, c * y - d * x);
            }
            ans = max(ans, ma - mi);
        }
    }
    println!("{}", ans);
}
