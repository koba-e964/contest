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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        u: i64, dd: i64, r: i64, l: i64, k: i64, p: i64,
        xs: usize1, ys: usize1, xt: usize1, yt: usize1,
        c: [chars; h],
    }
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), xs, ys));
    let mut dist = vec![vec![k + 1; w]; h];
    while let Some((Reverse(d), x, y)) = que.pop() {
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        if x > 0 && c[x - 1][y] != '#' {
            let to = u + if c[x - 1][y] == '@' { p } else { 0 };
            que.push((Reverse(d + to), x - 1, y));
        }
        if x < h - 1 && c[x + 1][y] != '#' {
            let to = dd + if c[x + 1][y] == '@' { p } else { 0 };
            que.push((Reverse(d + to), x + 1, y));
        }
        if y > 0 && c[x][y - 1] != '#' {
            let to = l + if c[x][y - 1] == '@' { p } else { 0 };
            que.push((Reverse(d + to), x, y - 1));
        }
        if y < w - 1 && c[x][y + 1] != '#' {
            let to = r + if c[x][y + 1] == '@' { p } else { 0 };
            que.push((Reverse(d + to), x, y + 1));
        }
    }
    println!("{}", if dist[xt][yt] <= k { "Yes" } else { "No" });
}
