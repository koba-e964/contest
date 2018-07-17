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
    let mut stdin = std::io::stdin();
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

type Res = HashMap<i64, i64>;

fn dfs(a: &[Vec<i64>], x: usize, y: usize, px: usize, py: usize,
       cur: i64,
       hm: &mut Res) {
    if px > x || py > y { return; }
    if x == px && y == py {
        *hm.entry(cur).or_insert(0) += 1;
        return;
    }
    if px < x {
        dfs(a, x, y, px + 1, py, cur ^ a[px + 1][py], hm);
    }
    if py < y {
        dfs(a, x, y, px, py + 1, cur ^ a[px][py + 1], hm);
    }
}

/// Enumerates all possible paths between (0,0) and (x, y).
/// Note that both values are xor'ed.
fn get_routes(a: &[Vec<i64>], x: usize, y: usize) -> Res {
    let mut ret = Res::new();
    dfs(a, x, y, 0, 0, a[0][0], &mut ret);
    ret
}

fn solve() {
    let n = get();
    let m = get();
    let k: i64 = get();
    let mut a = vec![vec![0i64; m]; n];
    for i in 0 .. n {
        for j in 0 .. m {
            a[i][j] = get();
        }
    }
    // Sqrt decomp (half-exhaustive search)
    let mut flip = vec![vec![0i64; m]; n];
    for i in 0 .. n {
        for j in 0 .. m {
            flip[i][j] = a[n - 1 - i][m - 1 - j];
        }
    }
    let mut tot = 0;
    for i in 0 .. min(n, m) {
        let t1 = get_routes(&a, n - 1 - i, i);
        let t2 = get_routes(&flip, i, m - 1 - i);
        for (sum1, f) in t1 {
            let rest = sum1 ^ k ^ a[n - 1 - i][i];
            if let Some(f2) = t2.get(&rest) {
                tot += f * f2;
            }
        }
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
