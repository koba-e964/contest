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

const INF: i32 = 1 << 28;

fn dfs(v: usize, par: usize, g: &[Vec<usize>]) -> [i32; 2] {
    let is_ok = g[v].len() >= 4;
    let mut ma = -INF;
    let mut maj = vec![];
    for &w in &g[v] {
        if w == par {
            continue;
        }
        let sub = dfs(w, v, g);
        if sub[1] > -INF {
            maj.push(sub[1]);
        }
        ma = ma.max(sub[0]);
    }
    let mut ret = [-INF; 2];
    if is_ok {
        maj.sort();
        maj.reverse();
        let mut s = 0;
        for i in 0..3.min(maj.len()) {
            s += maj[i];
        }
        ret[1] = s + 1;
        s = 0;
        for i in 0..4.min(maj.len()) {
            s += maj[i];
        }
        ma = ma.max(s + 1);
    }
    ret[0] = ma;
    ret
}

fn solve() {
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let res = dfs(0, n, &g)[0];
    if res == -INF {
        println!("-1");
    } else {
        println!("{}", res * 3 + 2);
    }
}
