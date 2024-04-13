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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, g: &Vec<Vec<usize>>, dep: &mut [usize]) {
    for &to in &g[v] {
        if to == par { continue; }
        dep[to] = dep[v] + 1;
        dfs(to, v, g, dep);
    }
}

// https://yukicoder.me/problems/no/2726 (3)
// 奇数深さの頂点の値からなる Nim だと思うと AC。
// Tags: nim, staircase-nim
fn solve() {
    let t: i32 = get();
    for _ in 0..t {
        let n: usize = get();
        let k: i64 = get();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let x = get::<usize>() - 1;
            let y = get::<usize>() - 1;
            g[x].push(y);
            g[y].push(x);
        }
        let a: Vec<i64> = (0..n).map(|_| get()).collect();
        let mut dep = vec![0; n];
        dfs(0, n, &g, &mut dep);
        let mut g = 0;
        for i in 1..n {
            if dep[i] % 2 == 1 {
                g ^= a[i] % (k + 1);
            }
        }
        println!("{}", if g == 0 { "P" } else { "K" })
    }
}
