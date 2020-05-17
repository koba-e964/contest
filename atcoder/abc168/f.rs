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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn dfs(x: usize, y: usize, dp: &mut [Vec<bool>],
       vert: &[Vec<i32>],
       hori: &[Vec<i32>]) {
    let n = dp.len();
    let m = dp[0].len();
    let mut que = VecDeque::new();
    que.push_back((x, y));
    // (_, _, kind (vert/hori), delta)
    let dxy = [(1, 0, 1, 1), (0, 1, 0, 1), (-1, 0, 1, 0), (0, -1, 0, 0)];
    while let Some((x, y)) = que.pop_front() {
        if dp[x][y] { continue; }
        dp[x][y] = true;
        for &(dx, dy, kind, delta) in &dxy {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= n || ny >= m { continue; }
            if kind == 1 {
                let wall = x.wrapping_add(delta as usize);
                if hori[wall][y] > 0 { continue; }
            } else {
                assert_eq!(kind, 0);
                let wall = y.wrapping_add(delta as usize);
                if vert[x][wall] > 0 { continue; }
            }
            que.push_back((nx, ny));
        }
    }
}

// Tags: stack-overflow coordinate-compression
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize,
        abc: [(i64, i64, i64); n],
        def: [(i64, i64, i64); m],
    }
    let mut xcoord = vec![0];
    let mut ycoord = vec![0];
    for &(a, b, c) in &abc {
        xcoord.push(a);
        xcoord.push(b);
        ycoord.push(c);
    }
    for &(a, b, c) in &def {
        xcoord.push(a);
        ycoord.push(b);
        ycoord.push(c);
    }
    xcoord.insert(0, -INF);
    xcoord.push(INF);
    ycoord.insert(0, -INF);
    ycoord.push(INF);
    xcoord.sort();
    xcoord.dedup();
    ycoord.sort();
    ycoord.dedup();
    const INF: i64 = 1 << 50;
    let p = xcoord.len();
    let q = ycoord.len();
    let mut dp = vec![vec![false; q - 1]; p - 1];
    let mut vert = vec![vec![0; q - 1]; p];
    let mut hori = vec![vec![0; q]; p - 1];
    for &(a, b, c) in &abc {
        let a = xcoord.binary_search(&a).unwrap();
        let b = xcoord.binary_search(&b).unwrap();
        let c = ycoord.binary_search(&c).unwrap();
        vert[a][c] += 1;
        vert[b][c] -= 1;
    }
    for &(a, b, c) in &def {
        let a = xcoord.binary_search(&a).unwrap();
        let b = ycoord.binary_search(&b).unwrap();
        let c = ycoord.binary_search(&c).unwrap();
        hori[a][b] += 1;
        hori[a][c] -= 1;
    }
    for i in 0..p - 1 {
        for j in 0..q - 1 {
            vert[i + 1][j] += vert[i][j];
            hori[i][j + 1] += hori[i][j];
        }
    }
    /*
    eprintln!("vert:");
    for i in 0..p - 1 {
        eprintln!("{:?}", vert[i]);
    }
    eprintln!("hori:");
    for i in 0..p - 1 {
        eprintln!("{:?}", hori[i]);
    }*/
    let x = xcoord.binary_search(&0).unwrap();
    let y = ycoord.binary_search(&0).unwrap();
    dfs(x, y, &mut dp, &vert, &hori);
    /*
    for i in 0..p - 1 {
        for j in 0..q - 1 {
            eprint!("{}", if dp[i][j] { '*' } else { '.' });
        }
        eprintln!();
    }*/
    let mut tot: i64 = 0;
    let mut inf = false;
    'outer:
    for i in 0..p - 1 {
        for j in 0..q - 1 {
            if !dp[i][j] { continue; }
            if i == 0 || i == p - 2 || j == 0 || j == q - 2 {
                inf = true;
                break 'outer;
            }
            tot += (xcoord[i + 1] - xcoord[i]) * (ycoord[j + 1] - ycoord[j]);
        }
    }
    if inf {
        puts!("INF\n");
        return;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
