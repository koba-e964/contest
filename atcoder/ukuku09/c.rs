use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const INF: i32 = 1 << 28;

type Int = i32;
fn ex_gcd(x: Int, y: Int) -> (Int, Int, Int) {
    if y == 0 {
        return (x, 1, 0);
    }
    let q = x / y;
    let r = x % y;
    let (res, a, b) = ex_gcd(y, r);
    let tmp = a - q * b;
    (res, b, tmp)
}

// Calculates the intersection of two arithmetic progressions,
// x[n] = a + b * n and y[n] = c + d * n (n >= 0).
// p1 = (a, b), p2 = (c, d)
fn arith_prog_intersect(p1: (Int, Int), p2: (Int, Int)) -> Option<(Int, Int)> {
    if p1.0 > p2.0 {
        return arith_prog_intersect(p2, p1);
    }
    let (g, _u, v) = ex_gcd(p1.1, p2.1);
    let lcm = p1.1 / g * p2.1;
    if (p1.0 - p2.0) % g != 0 {
        return None;
    }
    let mut diff = (p2.0 - p1.0) / g;
    diff *= -v % (p1.1 / g);
    diff %= p1.1 / g;
    if diff < 0 {
        diff += p1.1 / g;
    }
    let x = p2.0 + diff * p2.1;
    Some((x, lcm))
}

fn calc((x, xp): (i32, i32), (y, yp): (i32, i32)) -> i32 {
    if xp == 0 && yp == 0 {
        return if x == y { x } else { INF };
    }
    if xp == 0 {
        if x < y { return INF; }
        return (x - y + yp - 1) / yp * yp + y;
    }
    if yp == 0 {
        if x > y { return INF; }
        return (y - x + xp - 1) / xp * xp + x;
    }
    if let Some(v) = arith_prog_intersect((x, xp), (y, yp)) {
        v.0
    } else {
        INF
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize, q: usize,
        s: [chars; h],
        sg: [(usize1, usize1, usize1, usize1); q],
    }
    let mut all = vec![];
    for s in s {
        all.extend_from_slice(&s);
    }
    let mut dist = vec![vec![[INF; 2]; h * w]; h * w];
    let mut per = vec![vec![0; h * w]; h * w];
    let delta = [(1, '>'), (w as i32, 'v'), (-1, '<'), (-(w as i32), '^')];
    for i in 0..h * w {
        if all[i] == '#' || all[i] == '.' { continue; }
        for &(delta, ch) in &delta {
            if all[i] != ch { continue; }
            let new = (i as i32 + delta) as usize;
            if all[new] == '#' {
                all[i] = 's';
            }
        }
        
    }
    for i in 0..h * w {
        if all[i] == '#' { continue; }
        let mut que = VecDeque::new();
        que.push_back((0, if all[i] == '.' || all[i] == 's' { 1 } else { 0 }, i));
        while let Some((d, wait, v)) = que.pop_front() {
            if dist[i][v][wait] <= d {
                if wait == 0 && dist[i][v][wait] < d && per[i][v] == 0 {
                    per[i][v] = d - dist[i][v][wait];
                } else {
                    continue;
                }
            } else {
                dist[i][v][wait] = d;
            }
            for &(delta, ch) in &delta {
                if all[v] != '.' && all[v] != ch { continue; }
                let new = (v as i32 + delta) as usize;
                if all[new] != '#' {
                    let newwait = wait | if all[new] == '.' || all[new] == 's' { 1 } else { 0 };
                    que.push_back((d + 1, newwait, new));
                }
            }
        }
    }
    for (sx, sy, gx, gy) in sg {
        let s = sy * w + sx;
        let g = gy * w + gx;
        let mut mi = INF;
        for i in 0..h * w {
            let x = dist[s][i];
            let y = dist[g][i];
            for j in 0..2 {
                if x[j] >= INF { continue; }
                for k in 0..2 {
                    if y[k] >= INF { continue; }
                    if (j, k) == (1, 1) {
                        mi = min(mi, max(x[j], y[k]));
                        continue;
                    }
                    if (j, k) == (1, 0) {
                        let val = calc((x[j], 1), (y[k], per[g][i]));
                        mi = min(mi, val);
                        continue;
                    }
                    if (j, k) == (0, 1) {
                        let val = calc((x[j], per[s][i]), (y[k], 1));
                        mi = min(mi, val);
                        continue;
                    }
                    let val = calc((x[j], per[s][i]), (y[k], per[g][i]));
                    mi = min(mi, val);
                }
            }
        }
        puts!("{}\n", if mi >= INF { -1 } else { mi });
    }
}
