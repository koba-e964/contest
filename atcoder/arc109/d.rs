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

fn quo(x: i64, y: i64) -> i64 {
    let mut q = x / y;
    let r = x - q * y;
    if r < 0 {
        q -= 1;
    }
    q
}

fn sgn(x: i64) -> i64 {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

fn gen((x, y): (i64, i64), (dx, dy): (i64, i64)) -> Vec<((i64, i64), (i64, i64), i64)> {
    let mut ans = vec![((x, y), (dx, dy), 0)];
    for &p in &[-1, 1] {
        for &q in &[-1, 1] {
            ans.push(((x, y), (dx * p, dy * q), 1));
        }
    }
    ans.push(((x + dx, y), (-dx, dy), 1));
    ans.push(((x + dx, y), (-dx, -dy), 1));
    ans.push(((x, y + dy), (dx, -dy), 1));
    ans.push(((x, y + dy), (-dx, -dy), 1));
    ans.push(((x + dx, y + dy), (-dx, -dy), 2));
    ans
}

const INF: i64 = 1 << 50;

fn calc_in((x, y): (i64, i64), (dx, dy): (i64, i64)) -> i64 {
    if (x, y) == (0, 0) {
        return if (dx, dy) == (1, 1) {
            0
        } else {
            1
        };
    }
    if (dx, dy) != (1, 1) {
        return INF;
    }
    if x > 0 && y > 0 {
        return INF;
    }
    if x + y < 0 {
        return INF;
    }
    2 * max(x.abs(), y.abs())
}
fn calc(p: (i64, i64), d: (i64, i64)) -> i64 {
    let gp = gen(p, d);
    let ip = gen((0, 0), (-1, -1));
    let mut mi = INF;
    for &(p, d, x) in &gp {
        for (p, d, tx) in gen(p, d) {
        for &(ip, id, y) in &ip {
            for (ip, id, ty) in gen(ip, id) {
                let mut sx = p.0 - ip.0;
                let mut sy = p.1 - ip.1;
                sx *= id.0;
                sy *= id.1;
                let tmp = calc_in((sx, sy), (d.0 * id.0, d.1 * id.1));
                mi = min(mi, tmp + tx + x + ty + y);
            }
        }
        }
    }
    mi
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
        xy: [[(i64, i64); 3]],
    }
    for xy in xy {
        let mut x = 1;
        let mut y = 1;
        for i in 0..3 {
            x += xy[i].0;
            y += xy[i].1;
        }
        x = quo(x, 3);
        y = quo(y, 3);
        let mut dx = 3 * x;
        let mut dy = 3 * y;
        for i in 0..3 {
            dx -= xy[i].0;
            dy -= xy[i].1;
        }
        x = quo(2 * x - dx, 2);
        y = quo(2 * y - dy, 2);
        eprintln!("{} {} {} {}",x , y, dx, dy);
        puts!("{}\n", calc((x, y), (dx, dy)));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
