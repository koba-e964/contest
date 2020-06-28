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

fn score(c: &[i64], s: &[Vec<i64>], t: &[usize]) -> i64 {
    let d = t.len();
    let mut last = [0; 26];
    let mut sc = 0;
    for i in 0..d {
        last[t[i]] = i as i64 + 1;
        sc += s[i][t[i]];
        for j in 0..26 {
            sc -= (i as i64 - last[j] + 1) * c[j];
        }
    }
    sc
}

fn diff_score(c: &[i64], s: &[Vec<i64>], t: &mut [usize],
              pos1: usize, pos2: usize) -> i64 {
    if t[pos1] == t[pos2] {
        return 0;
    }
    let d = t.len();
    let mut last = [0; 26];
    let k1 = t[pos1];
    let k2 = t[pos2];
    let mut sc = s[pos1][k2] - s[pos1][k1];
    sc -= s[pos2][k2] - s[pos2][k1];
    for i in 0..d {
        last[t[i]] = i as i64 + 1;
        sc += (i as i64 - last[k1] + 1) * c[k1];
        sc += (i as i64 - last[k2] + 1) * c[k2];
    }
    last = [0; 26];
    t.swap(pos1, pos2);
    for i in 0..d {
        last[t[i]] = i as i64 + 1;
        sc -= (i as i64 - last[k1] + 1) * c[k1];
        sc -= (i as i64 - last[k2] + 1) * c[k2];
    }
    sc
}

fn diff_score_upd(c: &[i64], s: &[Vec<i64>], t: &mut [usize],
              pos: usize, to: usize) -> i64 {
    if t[pos] == to {
        return 0;
    }
    let d = t.len();
    let mut last = [0; 26];
    let k = t[pos];
    let mut sc = s[pos][to] - s[pos][k];
    for i in 0..d {
        last[t[i]] = i as i64 + 1;
        sc += (i as i64 - last[k] + 1) * c[k];
        sc += (i as i64 - last[to] + 1) * c[to];
    }
    last = [0; 26];
    t[pos] = to;
    for i in 0..d {
        last[t[i]] = i as i64 + 1;
        sc -= (i as i64 - last[k] + 1) * c[k];
        sc -= (i as i64 - last[to] + 1) * c[to];
    }
    sc
}

fn rnd_nxt(rnd: &mut u64) -> u32 {
    *rnd = rnd.wrapping_mul(0xdeadc0de01111011).wrapping_add(0x13331);
    (*rnd >> 32) as u32
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        d: usize,
        c: [i64; 26],
        s: [[i64; 26]; d],
    }
    let mut rnd = 1000;
    let mut t = vec![0; d];
    /*
    let mut ss = vec![];
    for i in 0..d {
        for j in 0..26 {
            ss.push((s[i][j], i, j));
        }
    }
    ss.sort();
    let mut rest = vec![d / 26; 26];
    for i in 0..d % 26 {
        rest[i] += 1;
    }
    while let Some((_x, i, j)) = ss.pop() {
        if rest[j] > 0 {
            t[i] = j;
            rest[j] -= 1;
        }
    }
    for i in 0..26 {
        assert_eq!(rest[i], 0);
    }
     */
    for i in 0..d {
        t[i] = i % 26;
    }
    let mut sc = score(&c, &s, &t);
    eprintln!("init sc = {}", sc);
    for _ in 0..80000 {
        let x = rnd_nxt(&mut rnd) as usize % d;
        let to = rnd_nxt(&mut rnd) as usize % 26;
        let old = t[x];
        let diff = diff_score_upd(&c, &s, &mut t, x, to);
        let newsc = sc + diff;
        if diff > 0 {
            sc = newsc
        } else {
            t[x] = old;
        }
    }
    eprintln!("prep sc = {}", sc);
    const TRIAL: usize = 1_100_000;
    for i in 0..TRIAL {
        let thresh = (TRIAL ) as i64 * 50 / TRIAL as i64;
        if i % 40000 == 0 {
            eprintln!("i = {}, sc = {}", i, sc);
        }
        let x = rnd_nxt(&mut rnd) as usize % d;
        let y = (x + rnd_nxt(&mut rnd) as usize % (d - 1) + 1) % d;
        let diff = diff_score(&c, &s, &mut t, x, y);
        /*
        let to = rnd_nxt(&mut rnd) as usize % 26;
        let old = t[x];
        let diff = diff_score_upd(&c, &s, &mut t, x, to);
         */
        let newsc = sc + diff;
        let drop = (rnd_nxt(&mut rnd) % 65536) as i64;
        if diff > -(thresh) {
            sc = newsc
        } else {
            t.swap(x, y);
        }
    }
    for i in 0..d {
        puts!("{}\n", t[i] + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
