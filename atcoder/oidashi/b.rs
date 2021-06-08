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

fn trans(s: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let h = s.len();
    let w = s[0].len();
    let mut ret = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            ret[j][i] = s[i][j];
        }
    }
    ret
}

const INF: i32 = 1 << 28;

fn play(h: usize, w: usize, tmp: &mut [Vec<i32>], broken: &[i32])
        -> Option<i32> {
    let mut cnt = 0;
    for i in 1..h {
        for j in 0..w {
            if tmp[i - 1][j] != 0 {
                if (broken[i] & 1 << j) != 0 {
                    return None;
                }
                tmp[i - 1][j] ^= 1;
                if i < h - 1 {
                    tmp[i + 1][j] ^= 1;
                }
                if j > 0 {
                    tmp[i][j - 1] ^= 1;
                }
                tmp[i][j] ^= 1;
                if j < w - 1 {
                    tmp[i][j + 1] ^= 1;
                }
                cnt += 1;
            }
        }
    }
    let mut pat = 0;
    for j in 0..w {
        pat |= tmp[h - 1][j] << j;
    }
    if pat == 0 {
        Some(cnt)
    } else {
        None
    }
}

fn calc(h: usize, w: usize, s: Vec<Vec<i32>>, xy: Vec<(usize, usize)>)
        -> i32 {
    assert!(w <= 20);
    let mut broken = vec![0; h];
    for &(x, y) in &xy {
        broken[x] |= 1 << y;
    }
    let mut pre = vec![INF; 1 << w];
    for bits in 0..1 << w {
        if (bits & broken[h - 1] as usize) != 0 { continue; }
        let v = (bits ^ bits << 1 ^ bits >> 1) & ((1 << w) - 1);
        pre[v] = min(pre[v], bits.count_ones() as i32);
    }
    let mut mi = INF;
    let mut tmp = vec![vec![0; w]; h];
    for bits in 0i32..1 << w {
        if (bits & broken[0]) != 0 { continue; }
        // Copy to avoid allocation
        for i in 0..h {
            for j in 0..w {
                tmp[i][j] = s[i][j];
            }
        }
        for i in 0..w {
            if (bits & 1 << i) != 0 {
                if i > 0 {
                    tmp[0][i - 1] ^= 1;
                }
                tmp[0][i] ^= 1;
                if i < w - 1 {
                    tmp[0][i + 1] ^= 1;
                }
                tmp[1][i] ^= 1;
            }
        }
        if let Some(score) = play(h, w, &mut tmp, &broken) {
            mi = min(mi, bits.count_ones() as i32 + score);
        }
    }
    mi
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize,
        s: [[i32; w]; h],
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut h = h;
    let mut w = w;
    let mut s = s;
    let mut xy = xy;
    for i in 0..n {
        let xy = &mut xy[i];
        std::mem::swap(&mut xy.0, &mut xy.1);
    }
    if h < w {
        std::mem::swap(&mut h, &mut w);
        s = trans(s);
        for i in 0..n {
            let xy = &mut xy[i];
            std::mem::swap(&mut xy.0, &mut xy.1);
        }
    }
    puts!("{}\n", calc(h, w, s, xy));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
