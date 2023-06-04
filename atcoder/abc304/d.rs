use std::cmp::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        _w: i64, _h: i64,
        n: usize,
        pq: [(i64, i64); n],
        aa: usize,
        a: [i64; aa],
        bb: usize,
        b: [i64; bb],
    }
    let mut hm = HashMap::new();
    for &(p, q) in &pq {
        let x = a.binary_search(&p).unwrap_err();
        let y = b.binary_search(&q).unwrap_err();
        *hm.entry((x, y)).or_insert(0) += 1;
    }
    let mut ma = 0;
    let mut mi = if (hm.len() as i64) < (aa as i64 + 1) * (bb as i64 + 1) {
        0
    } else {
        n as i64
    };
    for (_, v) in hm {
        ma = max(ma, v);
        mi = min(mi, v);
    }
    println!("{} {}", mi, ma);
}
