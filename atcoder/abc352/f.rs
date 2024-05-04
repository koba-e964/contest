use std::io::{Write, BufWriter};
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        abc: [(usize1, usize1, i32); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, -c));
        g[b].push((a, c));
    }
    let mut vis = vec![false; n];
    let mut pot = vec![0; n];
    let mut conn = vec![];
    let mut cb = vec![];
    let mut min = vec![];
    for i in 0..n {
        if vis[i] {
            continue;
        }
        let mut this = vec![];
        let mut que = std::collections::VecDeque::new();
        que.push_back(i);
        while let Some(v) = que.pop_front() {
            if vis[v] { continue; }
            vis[v] = true;
            this.push(v);
            for &(w, c) in &g[v] {
                pot[w] = pot[v] + c;
                que.push_back(w);
            }
        }
        let mut mi = (1, 0);
        for &v in &this {
            mi = mi.min((pot[v], v));
        }
        let mut bits = 0;
        for &v in &this {
            bits |= 1 << (pot[v] - mi.0);
        }
        conn.push(this);
        cb.push(bits);
        min.push(mi);
    }
    let mut ans = vec![0; n];
    for i in 0..conn.len() {
        let mut dp = vec![false; 1 << n];
        dp[0] = true;
        for j in 0..conn.len() {
            if i == j { continue; }
            let mut ep = vec![false; 1 << n];
            for bits in 0..1 << n {
                for k in 0..n {
                    if cb[j] << k >= 1 << n || (bits & cb[j] << k) != 0 { continue; }
                    ep[bits | cb[j] << k] |= dp[bits];
                }
            }
            dp = ep;
        }
        let mut poss = vec![];
        for j in 0..n {
            if cb[i] << j < 1 << n && dp[(1 << n) - 1 - (cb[i] << j)] {
                poss.push(j);
            }
        }
        if poss.len() >= 2 {
            for &v in &conn[i] {
                ans[v] = -1;
            }
        } else {
            for &v in &conn[i] {
                ans[v] = poss[0] as i32 + 1 + pot[v] - min[i].0;
            }
        }
    }
    putvec!(ans);
}
