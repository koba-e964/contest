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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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
        h: usize, w: usize,
        sx: usize1, sy: usize1, gx: usize1, gy: usize1,
        s: [chars; h],
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; 2 * h * w];
    let mut que = VecDeque::new();
    let sv = sx * w + sy;
    let gv = gx * w + gy;
    que.push_back((0, sv));
    que.push_back((0, sv + h * w));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        let ph = v / (h * w);
        let x = (v / w) % h;
        let y = v % w;
        if ph == 0 {
            // vertical
            if x > 0 && s[x - 1][y] == '.' {
                que.push_back((d + 1, v - w + h * w));
            }
            if x + 1 < h && s[x + 1][y] == '.' {
                que.push_back((d + 1, v + w + h * w));
            }
        } else {
            // horizontal
            if y > 0 && s[x][y - 1] == '.' {
                que.push_back((d + 1, v - 1 - h * w));
            }
            if y + 1 < w && s[x][y + 1] == '.' {
                que.push_back((d + 1, v + 1 - h * w));
            }
        }
    }
    let val = min(dist[gv], dist[gv + h * w]);
    puts!("{}\n", if val >= INF { -1 } else { val });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
