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

// Tags: memory-compaction, tight-memory-limit
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
        n: usize, m: usize, l: usize,
        s: chars,
        c: [chars; n],
    }
    const INF: i32 = 1 << 30;
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    let sz = (l + 1) * n * m * 8;
    let mut gcomp = vec![];
    let mut offset = vec![0; sz];
    let mut offset_now = 0;
    for i in 0..l + 1 {
        for x in 0..n {
            for y in 0..m {
                for d in 0..8 {
                    let (dx, dy) = dxy[d % 4];
                    let v = i * n * m * 8 + x * m * 8 + y * 8 + d;
                    offset[v] = offset_now;
                    if i >= l {
                        continue;
                    }
                    let nx = x.wrapping_add(dx as usize);
                    let ny = y.wrapping_add(dy as usize);
                    if nx >= n || ny >= m {
                        continue;
                    }
                    if c[x][y] != s[i] || d >= 4 {
                        if c[nx][ny] != s[i] {
                            gcomp.push((
                                (i * n * m * 8 + nx * m * 8 + ny * 8 + (d % 4)) as i32,
                                1));
                        } else {
                            for d2 in 0..4 {
                                if d % 4 == d2 {
                                    continue;
                                }
                                let v2 = (i + 1) * n * m * 8 + nx * m * 8 + ny * 8 + d2 + 4;
                                gcomp.push((v2 as i32, 1));
                            }
                        }
                    }
                    offset_now = gcomp.len();
                }
            }
        }
    }
    offset.push(offset_now);
    let mut dist = vec![INF; sz];
    let mut que = VecDeque::new();
    for x in 0..n {
        for y in 0..m {
            for d in 4..8 {
                que.push_back((0, (x * m * 8 + y * 8 + d) as i32));
            }
        }
    }
    while let Some((d, v)) = que.pop_front() {
        let v = v as usize;
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &(w, c) in &gcomp[offset[v]..offset[v + 1]] {
            if dist[w as usize] <= d + c {
                continue;
            }
            if c == 0 {
                que.push_front((d, w));
            } else {
                que.push_back((d + 1, w));
            }
        }
    }
    let mut mi = INF;
    for x in 0..n {
        for y in 0..m {
            for d in 0..4 {
                mi.chmin(dist[l * m * n * 8 + x * m * 8 + y * 8 + d + 4]);
            }
        }
    }
    puts!("{}\n", if mi >= INF { -1 } else { mi });
}
