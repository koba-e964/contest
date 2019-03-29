#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn calc(g: &[Vec<usize>], revg: &[Vec<usize>], cnt: &mut [usize]) {
    let n = revg.len();
    let mut outdeg = vec![0; n];
    let mut zeroback = vec![0; n]; // zeroback[v] = #{w | (v -> u) in g, deg[v] = 1}
    let mut zero = 0;
    let mut zs = HashSet::new();
    for v in 0..n {
        for &w in &revg[v] {
            if outdeg[w] == 0 {
                zero -= 1;
                zs.remove(&w);
                zeroback[v] += 1;
            }
            if outdeg[w] == 1 {
                for &k in &g[w] {
                    if k < v {
                        zeroback[k] -= 1;
                    }
                }
            }
            outdeg[w] += 1;
        }
        cnt[v] += zero;
        if zero == 1 {
            let &x = zs.iter().next().unwrap();
            cnt[v] += zeroback[x];
        }
        zero += 1;
        zs.insert(v);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        uv: [(usize1, usize1)],
    }
    let mut g = vec![vec![]; n];
    let mut revg = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        revg[v].push(u);
    }
    // find a topological order
    let mut ord = vec![];
    {
        let mut indeg = vec![0; n];
        let mut que = VecDeque::new();
        for i in 0..n {
            indeg[i] = revg[i].len();
            if revg[i].is_empty() {
                que.push_back(i);
            }
        }
        while let Some(v) = que.pop_front() {
            ord.push(v);
            for &w in &g[v] {
                indeg[w] -= 1;
                if indeg[w] == 0 {
                    que.push_front(w);
                }
            }
        }
    }
    assert_eq!(ord.len(), n);
    // Replace g and revg
    {
        let mut invord = vec![0; n];
        for i in 0..n {
            invord[ord[i]] = i;
        }
        for i in 0..n {
            g[i].clear();
            revg[i].clear();
        }
        for &(u, v) in &uv {
            let a = invord[u];
            let b = invord[v];
            g[a].push(b);
            revg[b].push(a);
        }
    }
    let mut cnt = vec![0; n];
    calc(&g, &revg, &mut cnt);
    cnt.reverse();
    for i in 0..n {
        for w in g[i].iter_mut() {
            *w = n - 1 - *w;
        }
        for w in revg[i].iter_mut() {
            *w = n - 1 - *w;
        }
    }
    g.reverse();
    revg.reverse();
    calc(&revg, &g, &mut cnt);
    cnt.reverse();
    let mut ans = 0;
    for i in 0..n {
        if cnt[i] <= 1 {
            ans += 1;
        }
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
