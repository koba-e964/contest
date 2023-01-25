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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
        s: [chars; n],
        q: usize,
        uv: [(usize1, usize1); q],
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![(INF, 0); n]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                dist[i][j] = (1, -a[j]);
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let (x1, y1) = dist[i][k];
                let (x2, y2) = dist[k][j];
                dist[i][j] = min(dist[i][j], (x1 + x2, y1 + y2));
            }
        }
    }
    for (u, v) in uv {
        let (x, y) = dist[u][v];
        if x >= INF {
            puts!("Impossible\n");
        } else {
            puts!("{} {}\n", x, -y + a[u]);
        }
    }
}

