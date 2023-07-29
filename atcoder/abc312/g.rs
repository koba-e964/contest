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

fn dfs(v: usize, par: usize, g: &[Vec<usize>], sz: &mut [i64]) -> i64 {
    let mut sum = 1;
    for &w in &g[v] {
        if w != par {
            sum += dfs(w, v, g, sz);
        }
    }
    sz[v] = sum;
    sum
}

fn solve() {
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut sz = vec![0; n];
    dfs(0, n, &g, &mut sz);
    let nn = n as i64;
    let mut ans = nn * (nn - 1) * (nn - 2) / 6;
    for i in 0..n {
        let mut rem = nn - 1;
        for &w in &g[i] {
            if sz[i] < sz[w] { continue; }
            rem -= sz[w];
            ans += sz[w] * (sz[w] - 1) / 2;
        }
        ans += rem * (rem - 1) / 2;
        ans -= (nn - 2) * (nn - 1) / 2;
    }
    println!("{}", ans);
}
