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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

const INF: i64 = 1 << 50;

// Tags: divide-into-entangled-ranges, trie, trie-less, vec-halving
// Editorial: https://refine-p.hatenablog.com/entry/2020/12/12/164350
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
        n: usize, m: i64,
        a: [i64; n],
        b: [i64; n],
    }
    const B: usize = 30;
    let mut rng = vec![];
    for bits in 0..1 << (n / 2) {
        let mut ax = 0;
        let mut bs = 0;
        for i in 0..n / 2 {
            if (bits & 1 << i) != 0 {
                ax ^= a[i];
                bs += b[i];
            }
        }
        rng.push((ax, bs));
    }
    rng.sort();
    let mut ent = vec![];
    // A kind of coordinate compression.
    {
        let mut nxt = vec![];
        let mut cur = -1;
        let mut ma = -1;
        for (a, b) in rng {
            if cur == a {
                ma = max(ma, b);
            } else {
                if cur >= 0 {
                    nxt.push((cur, ma));
                }
                cur = a;
                ma = b;
            }
        }
        if cur >= 0 {
            nxt.push((cur, ma));
        }
        ent.push(nxt);
    }
    for i in 0..B {
        let mut nxt = vec![];
        let mut pos = 0;
        while pos < ent[i].len() {
            if pos + 1 < ent[i].len() &&
                (ent[i][pos].0 ^ ent[i][pos + 1].0) == (1 << i) {
                    nxt.push((ent[i][pos].0, max(ent[i][pos].1, ent[i][pos + 1].1)));
                    pos += 2;
                } else {
                    nxt.push((ent[i][pos].0 & -(1 << (i + 1)), ent[i][pos].1));
                    pos += 1;
                }
        }
        // eprintln!("ent[i] = {:?}, {:?}", ent[i], nxt);
        ent.push(nxt);
    }
    let mut spl = vec![];
    {
        let mut v = m + 1;
        while v > 0 {
            let lsb = v & -v;
            v -= lsb;
            spl.push((v, (lsb - 1).count_ones() as usize));
        }
    }
    let mut ma = 0;
    for bits in 0..1 << (n - n / 2) {
        let mut ax = 0;
        let mut bs = 0;
        for i in n / 2..n {
            if (bits & 1 << (i - n / 2)) != 0 {
                ax ^= a[i];
                bs += b[i];
            }
        }
        for &(lo, len) in &spl {
            let l = (ax ^ lo) & -(1 << len);
            let r = l + (1 << len);
            let lidx = ent[len].lower_bound(&(l, -INF));
            let ridx = ent[len].lower_bound(&(r, -INF));
            if lidx < ridx {
                assert_eq!(lidx + 1, ridx, "{} {} {} [{}, {}) {:?}", len, lidx, ridx, l, r, &ent[len][..10]);
                ma = max(ma, bs + ent[len][lidx].1);
            }
        }
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
