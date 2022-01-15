use std::collections::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input!(a: usize, n: usize);
    const W: usize = 1_000_000;
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; W];
    let mut que = VecDeque::new();
    que.push_back((0, 1));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        if a * v < W {
            que.push_back((d + 1, a * v));
        }
        if v % 10 != 0 {
            let r = v % 10;
            let q = v / 10;
            let mut tmp = 1;
            while tmp <= q {
                tmp *= 10;
            }
            que.push_back((d + 1, r * tmp + q));
        }
    }
    println!("{}", if dist[n] >= INF { -1 } else { dist[n] });
}
