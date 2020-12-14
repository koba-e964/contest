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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/// https://judge.yosupo.jp/submission/5155
mod pollard_rho {
    use std::collections::HashMap;
    /// binary gcd
    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        if y == 0 { return x; }
        if x == 0 { return y; }
        let k = (x | y).trailing_zeros();
        y >>= k;
        x >>= x.trailing_zeros();
        while y != 0 {
            y >>= y.trailing_zeros();
            if x > y { let t = x; x = y; y = t; }
            y -= x;
        }
        x << k
    }

    fn add_mod(x: i64, y: i64, n: i64) -> i64 {
        let z = x + y;
        if z >= n { z - n } else { z }
    }

    fn mul_mod(x: i64, mut y: i64, n: i64) -> i64 {
        assert!(x >= 0);
        assert!(x < n);
        let mut sum = 0;
        let mut cur = x;
        while y > 0 {
            if (y & 1) == 1 {
                sum = add_mod(sum, cur, n);
            }
            cur = add_mod(cur, cur, n);
            y >>= 1;
        }
        sum
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 {
                prod = mul_mod(prod, cur, n);
            }
            e >>= 1;
            if e > 0 {
                cur = mul_mod(cur, cur, n);
            }
        }
        prod
    }

    pub fn is_prime(n: i64) -> bool {
        if n <= 1 { return false; }
        let small = [2, 3, 5, 7, 11, 13];
        if small.iter().any(|&u| u == n) { return true; }
        if small.iter().any(|&u| n % u == 0) { return false; }
        let mut d = n - 1;
        let e = d.trailing_zeros();
        d >>= e;
        // https://miller-rabin.appspot.com/
        let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        a.iter().all(|&a| {
            if a % n == 0 { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 { return true; }
            for _ in 0 .. e {
                if x == n - 1 {
                    return true;
                }
                x = mul_mod(x, x, n);
                if x == 1 { return false; }
            }
            x == 1
        })
    }

    fn pollard_rho(n: i64, c: &mut i64) -> i64 {
        // An improvement with Brent's cycle detection algorithm is performed.
        // https://maths-people.anu.edu.au/~brent/pub/pub051.html
        if n % 2 == 0 { return 2; }
        loop {
            let mut x: i64; // tortoise
            let mut y = 2; // hare
            let mut d = 1;
            let cc = *c;
            let f = |i| add_mod(mul_mod(i, i, n), cc, n);
            let mut r = 1;
            // We don't perform the gcd-once-in-a-while optimization
            // because the plain gcd-every-time algorithm appears to
            // outperform, at least on judge.yosupo.jp :)
            while d == 1 {
                x = y;
                for _ in 0..r {
                    y = f(y);
                    d = gcd((x - y).abs(), n);
                    if d != 1 {
                        break;
                    }
                }
                r *= 2;
            }
            if d == n {
                *c += 1;
                continue;
            }
            return d;
        }
    }

    /// Outputs (p, e) in p's ascending order.
    pub fn factorize(x: i64) -> Vec<(i64, usize)> {
        if x <= 1 {
            return Vec::new();
        }
        let mut hm = HashMap::new();
        let mut pool = vec![x];
        let mut c = 1;
        while let Some(u) = pool.pop() {
            if is_prime(u) {
                *hm.entry(u).or_insert(0) += 1;
                continue;
            }
            let p = pollard_rho(u, &mut c);
            pool.push(p);
            pool.push(u / p);
        }
        let mut v: Vec<_> = hm.into_iter().collect();
        v.sort();
        v
    }
} // mod pollard_rho

// Verified by https://atcoder.jp/contests/arc080/submissions/3957276
fn bipartite_matching(g: &[Vec<bool>]) -> (usize, Vec<Option<usize>>) {
    let n = g.len();
    if n == 0 { return (0, vec![]); }
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
    (ans, to)
}

fn calc(a: Vec<(i64, Vec<(i64, usize)>)>) -> (usize, Vec<i64>) {
    if a.is_empty() {
        return (0, vec![]);
    }
    //eprintln!("{:?}", a);
    let rem = a[0].0 % 6;
    if rem != 1 && rem != 5 {
        return (1, vec![a[0].0]);
    }
    assert_eq!(rem, 5);
    // We want a bipartite graph.
    // How should we separate primes into two classes?
    // 1 mod 6 vs 5 mod 6
    let mut coord_frm = vec![];
    let mut coord_lat = vec![];
    for &(_, ref pe) in &a {
        for &(p, _) in pe {
            if p % 6 <= 3 {
                coord_frm.push(p);
            } else {
                coord_lat.push(p);
            }
        }
    }
    coord_frm.sort(); coord_frm.dedup();
    coord_lat.sort(); coord_lat.dedup();
    let mut coord = coord_frm.clone();
    coord.extend_from_slice(&coord_lat);
    let m1 = coord_frm.len();
    let m2 = coord_lat.len();
    let mut g = vec![vec![false; m1 + m2]; m1 + m2];
    for &(_, ref pe) in &a {
        if pe.len() == 2 {
            let p = pe[0].0;
            let q = pe[1].0;
            if p % 6 <= 3 {
                let idx1 = coord_frm.binary_search(&p).unwrap();
                let idx2 = coord_lat.binary_search(&q).unwrap();
                g[idx1][m1 + idx2] = true;
            } else {
                let idx1 = coord_lat.binary_search(&p).unwrap();
                let idx2 = coord_frm.binary_search(&q).unwrap();
                g[idx2][m1 + idx1] = true;
            }
        } else {
            let p = pe[0].0;
            if p % 6 <= 3 {
                let idx = coord_frm.binary_search(&p).unwrap();
                g[idx][idx] = true;
            } else {
                let idx = m1 + coord_lat.binary_search(&p).unwrap();
                g[idx][idx] = true;
            }
        }
    }
    //eprintln!("g = {:?}", g);
    let (_ma, mtc) = bipartite_matching(&g);
    //eprintln!("ma = {:?}", ma);
    let mut res = vec![];
    for i in 0..m1 + m2 {
        if let Some(to) = mtc[i] {
            if to == i {
                res.push(coord[i] * coord[i] * coord[i]);
            } else {
                res.push(coord[to] * coord[i]);
            }
        }
    }
    (res.len(), res)
}

// Tags: maximum-matching, construction
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ma = (0, vec![]);
    let mut pes = vec![];
    for &a in &a {
        let pe = pollard_rho::factorize(a);
        pes.push((a, pe));
    }
    for rem in 0..6 {
        if rem == 1 {
            continue;
        }
        let mut sub = vec![];
        for &(a, ref pe) in &pes {
            if a % 6 != rem {
                continue;
            }
            let mut div = 1;
            for &(_p, e) in pe {
                div *= e + 1;
            }
            if div == 4 {
                sub.push((a, pe.clone()));
            }
        }
        let res = calc(sub);
        if ma < res {
            ma = res;
        }
    }
    if ma.0 == 0 {
        puts!("-1\n");
        return;
    }
    puts!("{}\n", ma.0);
    putvec!(ma.1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
