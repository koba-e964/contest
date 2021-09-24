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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [i64; n],
        xyz: [(usize1, usize1, i64); m],
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for &(x, y, z) in &xyz {
        dist[x][y] = min(dist[x][y], z);
        dist[y][x] = dist[x][y];
    }
    for l in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][l] + dist[l][j]);
            }
        }
    }
    let mut dp = vec![INF; 1 << n];
    for i in 0..n {
        dp[1 << i] = a[i];
    }
    for bits in 0usize..1 << n {
        if bits.count_ones() <= 1 { continue; }
        for i in 0..n {
            if (bits & 1 << i) == 0 { continue; }
            let pre = bits ^ 1 << i;
            let mut mi = INF;
            for j in 0..n {
                if i == j || (bits & 1 << j) == 0 { continue; }
                mi = min(mi, dist[j][i]);
            }
            dp[bits] = min(dp[bits], dp[pre] + a[i] + mi);
        }
    }
    let mut mi = INF;
    for bits in 0usize..1 << n {
        if bits.count_ones() == k as u32 {
            mi = min(mi, dp[bits]);
        }
    }
    println!("{}", mi);
}
