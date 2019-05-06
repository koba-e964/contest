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

fn dfs(d: &mut [Vec<i64>], verts: &[usize], cnt: &mut usize)
       -> Vec<(usize, usize, i64)> {
    if verts.len() <= 1 { return vec![]; }
    if verts.len() == 2 {
        let u = verts[0];
        let v = verts[1];
        return vec![(u, v, d[u][v])];
    }
    let mut diam = (-1, 0, 0);
    let mut edges = vec![];
    for &i in verts {
        for &j in verts {
            diam = max(diam, (d[i][j], j, i));
        }
    }
    let (diam, a, b) = diam;
    assert!(diam > 0);
    assert_ne!(a, b);
    let verts_elim: Vec<_> = verts.iter().cloned()
        .filter(|&x| x != a && x != b).collect();
    let mut hm = HashMap::<_, Vec<_>>::new();
    for &v in &verts_elim {
        assert_eq!((diam + d[v][a] + d[v][b]) % 2, 0);
        let x = (d[v][a] - d[v][b] + diam) / 2;
        let y = (d[v][a] + d[v][b] - diam) / 2;
        hm.entry(x).or_insert(vec![]).push((v, y));
    }
    let mut last = (a, 0);
    //eprintln!("vert = {:?}, hm = {:?}", verts, hm);
    let mut hm: Vec<(i64, Vec<_>)> = hm.into_iter().collect();
    hm.sort();
    for (x, vs) in hm {
        let zero = vs.iter().position(|&(_, d)| d == 0);
        let nv;
        if let Some(idx) = zero {
            nv = vs[idx].0;
        } else {
            nv = *cnt;
            *cnt += 1;
        }
        edges.push((last.0, nv, x - last.1));
        last = (nv, x);
        let mut new_verts = vec![nv];
        for &(v, dist) in &vs {
            if dist != 0 {
                d[v][nv] = dist;
                d[nv][v] = dist;
                new_verts.push(v);
            }
        }
        let sub = dfs(d, &new_verts, cnt);
        edges.extend_from_slice(&sub);
    }
    edges.push((last.0, b, diam - last.1));
    edges
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        k: usize,
        d: [[i64; k]; k],
    }
    const W: usize = 1000;
    let mut dd = vec![vec![0; W]; W];
    for i in 0..k {
        for j in 0..k {
            dd[i][j] = d[i][j];
        }
    }
    let mut cnt = k;
    let verts: Vec<_> = (0..k).collect();
    let edges = dfs(&mut dd, &verts, &mut cnt);
    //eprintln!("edges = {:?}", edges);
    puts!("{}\n", cnt);
    for &(u, v, d) in &edges {
        puts!("{} {} {}\n", u + 1, v + 1, d);
    }
    assert_eq!(edges.len() + 1, cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
