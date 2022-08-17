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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        tp: [(chars, i64); n],
    }
    let a = 27 * 27 * 27;
    let mut val = vec![0; a];
    for &(ref t, p) in &tp {
        let mut v = 0;
        let mut cur = a;
        for &c in t {
            v = 27 * v + (c as u8 - b'a' + 1) as usize;
            cur /= 27;
        }
        for i in 0..cur {
            val[v + a / cur * i] -= p;
        }
    }
    let mut edges = vec![];
    for y in 0..27 * 27 {
        for z in 1..27 {
            edges.push((y, y % 27 * 27 + z, val[y * 27 + z]));
        }
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; 27 * 27];
    dist[0] = 0;
    for _ in 0..27 * 27 - 1 {
        for &(x, y, c) in &edges {
            dist[y] = min(dist[y], dist[x] + c);
        }
    }
    let old = dist.clone();
    for &(x, y, c) in &edges {
        dist[y] = min(dist[y], dist[x] + c);
    }
    if dist != old {
        println!("Infinity");
        return;
    }
    let &mi = dist[1..].iter().min().unwrap();
    println!("{}", -mi);
}
