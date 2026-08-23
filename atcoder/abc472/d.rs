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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize, k: i32,
        s: [chars; h],
    }
    let mut row = vec![false; h];
    let mut col = vec![false; w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                row[i] = true;
                col[j] = true;
            }
        }
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if !row[i] && !col[j] {
                que.push_back((0, i, j));
            }
        }
    }
    let dxy = [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)];
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        for &(dx, dy) in &dxy {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= h || ny >= w {
                continue;
            }
            if s[nx][ny] == '.' {
                que.push_back((d + 1, nx, ny));
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= k {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
