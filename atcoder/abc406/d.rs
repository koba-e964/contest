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
        _h: usize, _w: usize, n: usize,
        xy: [(usize, usize); n],
        q: usize,
        tv: [(i32, usize); q],
    }
    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    for &(x, y) in &xy {
        rows.entry(x).or_insert(BTreeSet::new()).insert(y);
        cols.entry(y).or_insert(BTreeSet::new()).insert(x);
    }
    for (t, v) in tv {
        let mut tmp = vec![];
        if t == 1 {
            if let Some(e) = rows.get(&v) {
                for &y in e {
                    tmp.push((v, y));
                }
            }
        } else {
            if let Some(e) = cols.get(&v) {
                for &x in e {
                    tmp.push((x, v));
                }
            }
        }
        println!("{}", tmp.len());
        for &(x, y) in &tmp {
            if let Some(e) = rows.get_mut(&x) {
                e.remove(&y);
            }
            if let Some(e) = cols.get_mut(&y) {
                e.remove(&x);
            }
        }
    }
}
