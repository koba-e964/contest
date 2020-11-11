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

fn rot<T: Copy + Default>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = a.len();
    let m = a[0].len();
    let mut ret = vec![vec![T::default(); n]; m];
    for i in 0..n {
        for j in 0..m {
            ret[m - j - 1][i] = a[i][j];
        }
    }
    ret
}

fn calc(a: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let n = a.len();
    let m = a[0].len();
    let mut dp = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] != 2 && (a[i][j] == 1 || (j > 0 && dp[i][j - 1])) {
                dp[i][j] = true;
            }
        }
    }
    dp
}

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
        h: usize, w: usize, n: usize, m: usize,
        ab: [(usize1, usize1); n],
        cd: [(usize1, usize1); m],
    }
    let mut bl = vec![vec![0; w]; h];
    for &(a, b) in &ab {
        bl[a][b] = 1;
    }
    for &(c, d) in &cd {
        bl[c][d] = 2;
    }
    let dp1 = calc(&bl);
    let bl = rot(bl);
    let dp2 = rot(rot(rot(calc(&bl))));
    let bl = rot(bl);
    let dp3 = rot(rot(calc(&bl)));
    let bl = rot(bl);
    let dp4 = rot(calc(&bl));
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if dp1[i][j] || dp2[i][j] || dp3[i][j] || dp4[i][j] {
                cnt += 1;
            }
        }
    }
    puts!("{}\n", cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
