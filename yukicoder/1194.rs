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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, g: &[Vec<usize>], val: usize, dp: &mut [usize]) {
    if dp[v] >= val {
        return;
    }
    dp[v] = val;
    for &w in &g[v] {
        dfs(w, g, val, dp);
    }
}

fn solve() {
    input! {
        n: i64, m: usize,
        bc: [(i64, i64); m],
    }
    let mut coo = vec![];
    for &(b, c) in &bc {
        coo.push(b);
        coo.push(c);
    }
    coo.sort(); coo.dedup();
    let k = coo.len();
    let mut g = vec![vec![]; k];
    for &(b, c) in &bc {
        let b = coo.binary_search(&b).unwrap();
        let c = coo.binary_search(&c).unwrap();
        g[c].push(b);
    }
    let mut dp = vec![0; k];
    for i in (0..k).rev() {
        dfs(i, &g, i, &mut dp);
    }
    let mut ans = n * (n + 1) / 2;
    for i in 0..k {
        ans += coo[dp[i]] - coo[i];
    }
    println!("{}", ans);
}
