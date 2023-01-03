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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, g: &[Vec<usize>], vis: &mut [bool], c: usize, col: &mut [usize]) -> Result<[i64; 2], ()> {
    if vis[v] {
        if col[v] != c {
            return Err(());
        }
        return Ok([0; 2]);
    }
    vis[v] = true;
    col[v] = c;
    let mut cur = [0; 2];
    cur[c] += 1;
    for &w in &g[v] {
        let sub = dfs(w, g, vis, 1 - c, col)?;
        for i in 0..2 {
            cur[i] += sub[i];
        }
    }
    Ok(cur)
}

fn solve() {
    input! {
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut vis = vec![false; n];
    let mut col = vec![0; n];
    let mut ans = n as i64 * (n as i64 - 1) / 2;
    for i in 0..n {
        if !vis[i] {
            let sub = if let Ok(sub) = dfs(i, &g, &mut vis, 0, &mut col) {
                sub
            } else {
                println!("0");
                return;
            };
            for j in 0..2 {
                ans -= sub[j] * (sub[j] - 1) / 2;
            }
        }
    }
    println!("{}", ans - uv.len() as i64);
}

