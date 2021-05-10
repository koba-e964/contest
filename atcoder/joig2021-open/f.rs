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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        h: usize, w: usize, s: usize,
        a: [[usize1; w]; h],
    }
    const C: usize = 256;
    let mut lx = vec![h; C];
    let mut rx = vec![0; C];
    let mut ly = vec![w; C];
    let mut ry = vec![0; C];
    for i in 0..h {
        for j in 0..w {
            lx[a[i][j]].chmin(i);
            rx[a[i][j]].chmax(i);
            ly[a[i][j]].chmin(j);
            ry[a[i][j]].chmax(j);
        }
    }
    let mut l = vec![];
    let mut r = vec![];
    let mut v1 = vec![];
    let mut v2 = vec![];
    for i in 0..C {
        if lx[i] <= rx[i] {
            l.push(lx[i]);
            r.push(rx[i] + 1);
            v1.push(ly[i]);
            v2.push(ry[i] + 1);
        }
    }
    l.sort(); l.dedup();
    r.sort(); r.dedup();
    v1.sort(); v1.dedup();
    v2.sort(); v2.dedup();
    let mut leftup = vec![vec![[0u64; 4]; w + 1]; h + 1];
    let mut downright = vec![vec![[0u64; 4]; w + 1]; h + 1];
    let mut col = vec![vec![[0u64; 4]; w]; h];
    for i in 0..h {
        for j in 0..w {
            col[i][j][a[i][j] / 64] |= 1 << (a[i][j] % 64);
        }
    }
    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                leftup[i + 1][j + 1][k] = leftup[i + 1][j][k] | leftup[i][j + 1][k] | col[i][j][k];
            }
        }
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            for k in 0..4 {
                downright[i][j][k] = downright[i + 1][j][k] | downright[i][j + 1][k] | col[i][j][k];
            }
        }
    }
    let mut left = vec![[0u64; 4]; h + 1];
    let mut down = vec![[0u64; 4]; w + 1];
    let mut right = vec![[0u64; 4]; h + 1];
    let mut up = vec![[0u64; 4]; w + 1];
    for i in 0..h + 1 {
        left[i] = downright[i][0];
        right[i] = leftup[i][w];
    }
    for i in 0..w + 1 {
        down[i] = leftup[h][i];
        up[i] = downright[0][i];
    }
    let get_sum = |l: usize, r: usize, v1: usize, v2: usize| {
        let mut used = right[l];
        for k in 0..4 {
            used[k] |= down[v1][k];
        }
        for k in 0..4 {
            used[k] |= left[r][k];
        }
        for k in 0..4 {
            used[k] |= up[v2][k];
        }
        let mut sum = 0;
        for k in 0..4 {
            sum += used[k].count_ones();
        }
        sum as usize
    };
    let mut mi = C;
    for &l in &l {
        for &r in &r {
            if l >= r {
                continue;
            }
            let lim = s / (r - l);
            for &v1 in &v1 {
                let v2 = min(w, v1 + lim);
                mi.chmin(get_sum(l, r, v1, v2));
            }
        }
    }
    for &v1 in &v1 {
        for &v2 in &v2 {
            if v1 >= v2 {
                continue;
            }
            let lim = s / (v2 - v1);
            for &l in &l {
                let r = min(h, l + lim);
                mi.chmin(get_sum(l, r, v1, v2));
            }
        }
    }
    puts!("{}\n", mi);
}
