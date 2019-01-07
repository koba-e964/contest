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

fn prime_sieve(n: usize) -> Vec<bool> {
    let mut ans = vec![true; n];
    ans[0] = false;
    ans[1] = false;
    for i in 2 .. n {
        if !ans[i] { continue; }
        for j in 2 .. (n - 1) / i + 1 {
            ans[i * j] = false;
        }
    }
    ans
}

// Verified by https://atcoder.jp/contests/arc080/submissions/3957276
fn bipartite_matching(g: &[Vec<bool>]) -> usize {
    let n = g.len();
    if n == 0 { return 0; }
    let m = g[0].len();
    let mut to = vec![None; m];
    let mut visited = vec![false; n];
    let mut ans = 0;
    fn augment(v: usize, g: &[Vec<bool>],
               visited: &mut [bool], to: &mut [Option<usize>])
               -> bool {
        if visited[v] { return false; }
        visited[v] = true;
        for i in 0 .. g[v].len() {
            if !g[v][i] { continue; }
            if let Some(w) = to[i] {
                if augment(w, g, visited, to) {
                    to[i] = Some(v); return true;
                }
            } else {
                to[i] = Some(v); return true;
            }
        }
        false
    }
    for i in 0 .. n {
        for v in visited.iter_mut() { *v = false; }
        if augment(i, &g, &mut visited, &mut to) { ans += 1; }
    }
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! { x: [i64] }
    const W: usize = 1 << 24;
    let is_prime = prime_sieve(W);
    let mut hs = HashSet::new();
    for x in x {
        if hs.contains(&x) {
            hs.remove(&x);
        } else {
            hs.insert(x);
        }
        let x = x + 1;
        if hs.contains(&x) {
            hs.remove(&x);
        } else {
            hs.insert(x);
        }
    }
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for x in hs {
        if x % 2 == 0 { even.push(x); }
        else { odd.push(x); }
    }
    let n = even.len();
    let m = odd.len();
    let mut g = vec![vec![false; m]; n];
    for i in 0 .. n {
        for j in 0 .. m {
            if is_prime[(even[i] - odd[j]).abs() as usize] {
                g[i][j] = true;
            }
        }
    }
    let size = bipartite_matching(&g);
    let mut tot = 3 * ((n - size) % 2);
    tot += 2 * ((n - size) / 2);
    tot += 2 * ((m - size) / 2);
    tot += size;
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
