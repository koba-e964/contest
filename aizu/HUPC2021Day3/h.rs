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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        g: [chars; h],
        m: usize,
        abt: [(usize1, usize1, i64); m],
    }
    const INF: i64 = 1 << 50;
    let mut block = vec![vec![INF; w]; h];
    for (a, b, t) in abt {
        block[a][b] = t;
    }
    let mut dist = vec![vec![vec![INF; w]; h]; 2];
    let mut s = (0, 0, 0);
    let mut tr = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == 'S' {
                s = (0, i, j);
            }
            if g[i][j] == 'V' {
                tr = (i, j);
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back((0, s.0, s.1, s.2));
    let dxy = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    while let Some((d, z, x, y)) = que.pop_front() {
        if dist[z][x][y] <= d { continue; }
        dist[z][x][y] = d;
        let mut to = vec![];
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < h && ny < w && g[nx][ny] != '#' && block[nx][ny] > d + 1 {
                to.push((z, nx, ny, 1));
            }
        }
        if (x, y) == tr && z == 0 {
            to.push((1, x, y, 0));
        }
        for (nz, nx, ny, cost) in to {
            if cost == 0 {
                que.push_front((d, nz, nx, ny));
            } else {
                que.push_back((d + 1, nz, nx, ny));
            }
        }
    }
    println!("{}", if dist[1][s.1][s.2] >= INF { "No" } else { "Yes" });
}
