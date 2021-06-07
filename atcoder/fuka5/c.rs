#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn solve() {
    loop {
        let w: usize = get();
        let h: usize = get();
        let p: usize = get();
        if w == 0 {
            break;
        }
        let z: Vec<Vec<i64>> = (0..h).map(|_|
                                          (0..w).map(|_| get()).collect())
            .collect();
        let xy: Vec<(usize, usize)> = (0..p).map(|_| (get(), get())).collect();
        let mut vis = vec![vec![false; w]; h];
        let mut que = VecDeque::new();
        for &(x, y) in &xy {
            que.push_back((y, x));
        }
        let dxy = [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)];
        while let Some((a, b)) = que.pop_front() {
            if vis[a][b] {
                continue;
            }
            vis[a][b] = true;
            for &(dx, dy) in &dxy {
                let na = a.wrapping_add(dx as usize);
                let nb = b.wrapping_add(dy as usize);
                if na >= h || nb >= w {
                    continue;
                }
                if z[a][b] > z[na][nb] {
                    que.push_back((na, nb));
                }
            }
        }
        let mut ans = 0;
        for i in 0..h {
            for j in 0..w {
                if vis[i][j] {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
