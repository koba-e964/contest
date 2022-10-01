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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    let (mut sx, mut sy, mut ax, mut ay, mut gx, mut gy) = (0, 0, 0, 0, 0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 's' {
                sx = i;
                sy = j;
            }
            if s[i][j] == 'a' {
                ax = i;
                ay = j;
            }
            if s[i][j] == 'g' {
                gx = i;
                gy = j;
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back((0, sx, sy, ax, ay));
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![vec![vec![INF; w]; h]; w]; h];
    let mut ans = INF;
    let dxy = [(0i32, 1i32), (0, -1), (-1, 0), (1, 0)];
    while let Some((d, x, y, ax, ay)) = que.pop_front() {
        if dist[x][y][ax][ay] <= d { continue; }
        dist[x][y][ax][ay] = d;
        if (ax, ay) == (gx, gy) {
            ans = min(ans, d);
        }
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w { continue; }
            if s[nx][ny] == '#' { continue; }
            if (nx, ny) == (ax, ay) {
                let mx = (nx as i32 + dx) as usize;
                let my = (ny as i32 + dy) as usize;
                if mx >= h || my >= w { continue; }
                if s[mx][my] == '#' { continue; }
                que.push_back((d + 1, nx, ny, mx, my));
            } else {
                que.push_back((d + 1, nx, ny, ax, ay));
            }
        }
    }
    println!("{}", if ans >= INF { -1 } else { ans });
}
