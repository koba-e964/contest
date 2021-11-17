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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/340 (3)
// BFS で最短路。ただし辺を愚直に張ると TLE なので注意。
fn main() {
    input! {
        w: usize, h: usize, n: usize,
        b: [([usize], usize); n],
    }
    let mut g = vec![vec![]; h * w];
    let mut row = vec![vec![0; w]; h];
    let mut col = vec![vec![0; h]; w];
    for (mut b, blast) in b {
        b.push(blast);
        let m = b.len();
        for i in 0..m - 1 {
            let (x, y) = if b[i] < b[i + 1] {
                (b[i], b[i + 1])
            } else {
                (b[i + 1], b[i])
            };
            if x % w == y % w {
                col[x % w][x / w] += 1;
                col[x % w][y / w] -= 1;
            } else {
                row[x / w][x % w] += 1;
                row[x / w][y % w] -= 1;
            }
        }
    }
    for i in 0..h {
        for j in 0..w - 1 {
            row[i][j + 1] += row[i][j];
            if row[i][j] > 0 {
                g[i * w + j].push(i * w + j + 1);
                g[i * w + j + 1].push(i * w + j);
            }
        }
    }
    for i in 0..w {
        for j in 0..h - 1 {
            col[i][j + 1] += col[i][j];
            if col[i][j] > 0 {
                g[j * w + i].push(j * w + i + w);
                g[j * w + i + w].push(j * w + i);
            }
        }
    }
    let mut que = VecDeque::new();
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; h * w];
    que.push_back((0, 0));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &w in &g[v] {
            if dist[w] > d + 1 {
                que.push_back((d + 1, w));
            }
        }
    }
    if dist[h * w - 1] >= INF {
        println!("Odekakedekinai..");
    } else {
        println!("{}", dist[h * w - 1]);
    }
}
