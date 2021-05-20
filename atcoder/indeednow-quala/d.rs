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

fn bfs(maxdep: usize, v: &[i32], w: usize) -> HashMap<Vec<i32>, usize> {
    let h = v.len() / w;
    let mut dist = HashMap::new();
    let mut que = VecDeque::new();
    que.push_back((0, v.to_vec()));
    let dxy = [(0i32, 1i32), (-1, 0), (0, -1), (1, 0)];
    while let Some((d, mut v)) = que.pop_front() {
        if let Some(&md) = dist.get(&v) {
            if md <= d {
                continue;
            }
        }
        dist.insert(v.clone(), d);
        let idx = v.iter().position(|&x| x == 0).unwrap();
        let x = idx / w;
        let y = idx % w;
        if d < maxdep {
            for &(dx, dy) in &dxy {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx >= h || ny >= w {
                    continue;
                }
                let idx2 = nx * w + ny;
                v.swap(idx, idx2);
                que.push_back((d + 1, v.clone()));
                v.swap(idx, idx2);
            }
        }
    }
    dist
}

// Tags: bfs, sqrt-decomposition, half-enumeration
fn solve() {
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
        h: usize, w: usize,
        c: [i32; h * w],
    }
    let mut init = vec![0; h * w];
    for i in 0..h * w - 1 {
        init[i] = i as i32 + 1;
    }
    let fst = bfs(12, &c, w);
    let snd = bfs(12, &init, w);
    let mut mi = 24;
    for (v, d) in fst {
        if let Some(&d2) = snd.get(&v) {
            mi.chmin(d + d2);
        }
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
