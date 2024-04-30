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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        p: [[i64; n]; n],
        r: [[i64; n - 1]; n],
        d: [[i64; n]; n - 1],
    }
    let mut coo = vec![];
    for i in 0..n {
        for j in 0..n {
            coo.push(p[i][j]);
        }
    }
    coo.push(0);
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut mapped = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            mapped[i][j] = coo.binary_search(&p[i][j]).unwrap();
        }
    }
    let mut dist = vec![(INF, Reverse(INF)); n * n * m];
    const INF: i64 = 1 << 58;
    dist[0] = (0, Reverse(0));
    for x in 0..n {
        for y in 0..n {
            for k in 0..m {
                let idx = (x * n + y) * m + k;
                let (cost, Reverse(money)) = dist[idx];
                let nk = k.max(mapped[x][y]);
                let ma = coo[nk];
                let nidx = (x * n + y) * m + nk;
                if x + 1 < n {
                    let q = if money >= d[x][y] {
                        0
                    } else {
                        (d[x][y] - money + ma - 1) / ma
                    };
                    dist[nidx + n * m] = dist[nidx + n * m].min((cost + q + 1, Reverse(money + q * ma - d[x][y])));
                }
                if y + 1 < n {
                    let q = if money >= r[x][y] {
                        0
                    } else {
                        (r[x][y] - money + ma - 1) / ma
                    };
                    dist[nidx + m] = dist[nidx + m].min((cost + q + 1, Reverse(money + q * ma - r[x][y])));
                }
            }
        }
    }
    let mut mi = INF;
    for i in 0..m {
        mi = mi.min(dist[(n * n - 1) * m + i].0);
    }
    println!("{}", mi);
}
