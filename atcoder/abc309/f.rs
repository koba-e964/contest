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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://atcoder.jp/contests/abc309/tasks/abc309_f
// 2D LIS と似たような問題だが、LIS の長さ >= 2 かどうかだけ判定すれば良い。
// これは今まで見た点列を左上から右下に伸びていく点の集合として保持しておくことで、判定できる。
// Tags: 2d-lis
fn main() {
    input! {
        n: usize,
        a: [[i64; 3]; n],
    }
    let mut a = a;
    for i in 0..n {
        a[i].sort();
    }
    let mut pts = vec![];
    let mut coo1 = vec![];
    let mut coo2 = vec![];
    for i in 0..n {
        pts.push((a[i][2], -a[i][0], -a[i][1]));
        coo1.push(a[i][0]);
        coo2.push(a[i][1]);
    }
    pts.sort();
    coo1.sort(); coo1.dedup();
    coo2.sort(); coo2.dedup();
    let mut bs = BTreeSet::new();
    const INF: i64 = 1 << 50;
    for (_t, x, mut y) in pts {
        let x = -x;
        let mut it = bs.range(..(x, -INF)).rev();
        if let Some(&(v1, v2)) = it.next() {
            if v2 > y {
                println!("Yes");
                return;
            }
        }
        loop {
            let mut it = bs.range((x + 1, -INF)..);
            if let Some(&(v1, v2)) = it.next() {
                if v2 < y {
                    bs.remove(&(v1, v2));
                    continue;
                }
            }
            break;
        }
        let mut it = bs.range((x, -INF)..(x, INF));
        if let Some(&val) = it.next() {
            y = max(y, val.1);
            bs.remove(&val);
        }
        bs.insert((x, y));
    }
    println!("No");
}
