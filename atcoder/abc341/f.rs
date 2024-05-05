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
    input! {
        n: usize, m: usize,
        uv: [(usize, usize); m],
        w: [usize; n],
        a: [i64; n],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        if w[u - 1] < w[v - 1] {
            g[v - 1].push(u - 1);
        }
        if w[u - 1] > w[v - 1] {
            g[u - 1].push(v - 1);
        }
    }
    let mut ord = vec![];
    for i in 0..n {
        ord.push((w[i], i));
    }
    ord.sort();
    let mut dp = vec![0i64; n];
    const W: usize = 5001;
    for (_, v) in ord {
        let mut ep = vec![0i64; W];
        for &u in &g[v] {
            for i in (w[u]..W).rev() {
                ep[i] = ep[i].max(ep[i - w[u]] + dp[u]);
            }
        }
        dp[v] = ep[w[v] - 1] + 1;
    }
    let mut tot = 0;
    for i in 0..n {
        tot += dp[i] * a[i];
    }
    println!("{}", tot);
}
