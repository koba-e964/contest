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
        a: usize1, sx: usize, sy: usize,
        b: usize1, gx: usize, gy: usize,
        c: [chars; h],
    }
    const W: usize = 2000;
    let mut vis = vec![vec![vec![false; W]; w]; h];
    let mut que = VecDeque::new();
    let dxy = [(0i32, 1i32), (1, 0), (-1, 0), (0, -1)];
    que.push_back((sx, sy, a));
    while let Some((x, y, k)) = que.pop_front() {
        if vis[x][y][k] { continue; }
        vis[x][y][k] = true;
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w { continue; }
            if c[nx][ny] == '*' {
                if k + 1 < W {
                    if vis[nx][ny][k + 1] { continue; }
                    que.push_back((nx, ny, k + 1));
                }
            } else {
                if k > 0 {
                    if vis[nx][ny][k - 1] { continue; }
                    que.push_back((nx, ny, k - 1));
                }
            }
        }
    }
    println!("{}", if vis[gx][gy][b] { "Yes" } else { "No" });
}
