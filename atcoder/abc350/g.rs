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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

const MOD: i64 = 998_244_353;

fn dfs(v: usize, p: usize, g: &[Vec<usize>], par: &mut [usize],
    newroot: usize, root: &mut [usize]) {
    par[v] = p;
    root[v] = newroot;
    for &w in &g[v] {
        if w == p {
            continue;
        }
        dfs(w, v, g, par, newroot, root);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        qs: [(i64, i64, i64); q],
    }
    let mut sz = vec![1; n];
    let mut root = vec![0; n];
    for i in 0..n {
        root[i] = i;
    }
    let mut par = vec![n; n];
    let mut g = vec![vec![]; n];
    let mut x = 0i64;
    for (ac, bc, cc) in qs {
        let a = (ac * (1 + x)) % MOD % 2;
        let b = ((bc * (1 + x)) % MOD % (n as i64)) as usize;
        let c = ((cc * (1 + x)) % MOD % (n as i64)) as usize;
        if a == 0 {
            let mut b = b;
            let mut c = c;
            g[b].push(c);
            g[c].push(b);
            let mut rb = root[b];
            let mut rc = root[c];
            if sz[rb] < sz[rc] {
                std::mem::swap(&mut rb, &mut rc);
                std::mem::swap(&mut b, &mut c);
            }
            dfs(c, b, &g, &mut par, rb, &mut root);
            sz[rb] += sz[rc];
            sz[rc] = 0;
        } else {
            let mut ok = None;
            if root[b] == root[c] {
                if par[b] != n && par[b] == par[c] {
                    ok = Some(par[b]);
                }
                if par[b] != n && par[par[b]] == c {
                    ok = Some(par[b]);
                }
                if par[c] != n && par[par[c]] == b {
                    ok = Some(par[c]);
                }
            }
            if let Some(v) = ok {
                puts!("{}\n", v + 1);
                x = v as i64 + 1;
            } else {
                puts!("0\n");
                x = 0;
            }
        }
    }
}
