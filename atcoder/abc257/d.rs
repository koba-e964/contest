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

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n],
    }
    let mut ans = 0i64;
    for bit in (0..32).rev() {
        let x = ans | 1 << bit;
        let mut dist = vec![vec![1; n]; n];
        for i in 0..n {
            let (xi, yi, pi) = xyp[i];
            for j in 0..n {
                let (xj, yj, _) = xyp[j];
                let tmp = (xi - xj).abs() + (yi - yj).abs();
                if tmp <= x * pi {
                    dist[i][j] = 0;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
                }
            }
        }
        if (0..n).all(|i| dist[i].iter().any(|&j| j != 0)) {
            ans = x;
        }
    }
    println!("{}", ans + 1);
}
