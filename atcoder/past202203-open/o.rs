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

fn dfs(v: usize, vis: &mut [bool], col: &mut [bool], c: bool, g: &[Vec<usize>]) -> (usize, usize, bool) {
    if vis[v] {
        return (0, 0, c == col[v]);
    }
    vis[v] = true;
    col[v] = c;
    let mut sz0 = 1;
    let mut sz1 = 0;
    let mut ok = true;
    for &w in &g[v] {
        let (subsz0, subsz1, subok) = dfs(w, vis, col, !c, g);
        sz0 += subsz1;
        sz1 += subsz0;
        ok &= subok;
    }
    (sz0, sz1, ok)
}

fn solve() {
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut vis = vec![false; n];
    let mut col = vec![false; n];
    let mut a = 0;
    let mut b = vec![];
    for i in 0..n {
        if !vis[i] {
            let (sz0, sz1, ok) = dfs(i, &mut vis, &mut col, false, &g);
            if !ok {
                a += sz0 + sz1;
            } else {
                b.push((sz0, sz1));
            }
        }
    }
    if a > n / 3 {
        println!("No");
        return;
    }
    let st = (n + 2) / 3;
    let rem = n / 3 - a;
    let mut dp = vec![vec![false; rem + 1]; st + 1];
    dp[0][0] = true;
    for (x, y) in b {
        let mut ep = vec![vec![false; rem + 1]; st + 1];
        for i in 0..st + 1 {
            for j in 0..rem + 1 {
                if !dp[i][j] { continue; }
                if i + x <= st {
                    ep[i + x][j] = true;
                }
                if i + y <= st {
                    ep[i + y][j] = true;
                }
                if j + x + y <= rem {
                    ep[i][j + x + y] = true;
                }
            }
        }
        dp = ep;
    }
    if dp[st][rem] {
        println!("Yes");
        return;
    }
    println!("No");
}
