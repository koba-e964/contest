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
        c: [chars; h],
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![INF; w]; h];
    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                s = (i, j);
            }
            if c[i][j] == 'G' {
                g = (i, j);
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back((0, s.0, s.1));
    let dxy = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d { continue; }
        dist[x][y] = d;
        let t = c[x][y];
        let mut to = vec![];
        if t == 'S' || t == 'G' || t == '.' {
            for &(dx, dy) in &dxy {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < h && ny < w && c[nx][ny] != '#' {
                    to.push((nx, ny, 1));
                }
            }
        } else {
            let (dx, dy) = match t {
                'U' => (-1, 0),
                'R' => (0, 1),
                'L' => (0, -1),
                'D' => (1, 0),
                _ => panic!(),
            };
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < h && ny < w && c[nx][ny] != '#' {
                to.push((nx, ny, 0));
            }
        }
        for (nx, ny, cost) in to {
            if cost == 0 {
                que.push_front((d, nx, ny));
            } else {
                que.push_back((d + 1, nx, ny));
            }
        }
    }
    let d = dist[g.0][g.1];
    println!("{}", if d >= INF { -1 } else { d });
}
