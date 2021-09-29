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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
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

fn dfs1(v: usize, par: usize, g: &[Vec<usize>], s: &[char],
        c: &mut [i64], w: &mut [i64]) {
    if s[v] == 'c' {
        c[v] += 1;
    } else {
        w[v] += 1;
    }
    for &ww in &g[v] {
        if ww == par { continue; }
        dfs1(ww, v, g, s, c, w);
        c[v] += c[ww];
        w[v] += w[ww];
    }
}

fn dfs2(v: usize, par: usize, g: &[Vec<usize>], s: &[char],
        c: &[i64], w: &[i64]) -> i64 {
    let mut ans = 0;
    let mut sc = c[0];
    let mut sw = w[0];
    if s[v] == 'w' {
        sw -= 1;
    } else {
        sc -= 1;
    }
    if s[v] == 'w' {
        ans += sc * sw;
    }
    for &ww in &g[v] {
        if ww == par { continue; }
        ans += dfs2(ww, v, g, s, c, w);
        if s[v] == 'w' {
            ans -= c[ww] * w[ww];
        }
        sc -= c[ww];
        sw -= w[ww];
    }
    if s[v] == 'w' {
        ans -= sc * sw;
    }
    ans
}

fn solve() {
    input! {
        n: usize,
        s: chars,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut c = vec![0i64; n];
    let mut w = vec![0i64; n];
    dfs1(0, n, &g, &s, &mut c, &mut w);
    println!("{}", dfs2(0, n, &g, &s, &c, &w));
}
