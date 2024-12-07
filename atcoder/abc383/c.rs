use std::collections::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let h: usize = get();
    let w: usize = get();
    let targ_d: i32 = get();
    let mut s = vec![];
    for _ in 0..h {
        s.push(get_word().chars().collect::<Vec<_>>());
    }
    const INF: i32 = 1 << 30;
    let mut dist = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                que.push_back((0, i, j));
            }
        }
    }
    let mut ans = 0;
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        if d <= targ_d {
            ans += 1;
        }
        for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= h || ny >= w {
                continue;
            }
            if s[nx][ny] == '#' {
                continue;
            }
            que.push_back((d + 1, nx, ny));
        }
    }
    println!("{ans}");
}
