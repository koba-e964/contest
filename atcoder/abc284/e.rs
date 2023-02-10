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

const LIM: i32 = 1_000_000;

fn dfs(v: usize, g: &[Vec<usize>], seen: &mut [bool], cnt: &mut i32) {
    if seen[v] {
        return;
    }
    seen[v] = true;
    *cnt += 1;
    if *cnt >= LIM {
        return;
    }
    for &w in &g[v] {
        dfs(w, g, seen, cnt);
        if *cnt >= LIM {
            return;
        }
    }
    seen[v] = false;
}

fn solve() {
    input! {
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut seen = vec![false; n];
    let mut cnt = 0;
    dfs(0, &g, &mut seen, &mut cnt);
    println!("{}", cnt);
}
