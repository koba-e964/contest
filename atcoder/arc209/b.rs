#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
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

#[allow(unused)]
trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn intersperse(f: &[usize]) -> String {
    let &ma = f.iter().max().unwrap();
    let mut idx = 0;
    for i in 0..26 {
        if f[i] == ma {
            idx = i;
        }
    }
    let mut buc = vec![vec![]; ma];
    let mut pos = 0;
    for i in 0..26 {
        if i != idx {
            for _ in 0..f[i] {
                buc[pos % ma].push(i);
                pos += 1;
            }
        }
    }
    let mut s = String::new();
    for i in 0..ma {
        s.push((b'a' + idx as u8) as char);
        for &v in &buc[i] {
            s.push((b'a' + v as u8) as char);
        }
    }
    s
}

fn solve() {
    #[allow(unused)]
    let out = std::io::stdout();
    #[allow(unused)]
    let mut out = BufWriter::new(out.lock());
    #[allow(unused)]
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
        t: usize,
        s: [chars; t],
    }
    for s in s {
        let n = s.len();
        const W: usize = 26;
        let mut f = vec![0; W];
        for c in s {
            f[(c as u8 - b'a') as usize] += 1usize;
        }
        let &ma = f.iter().max().unwrap();
        let mut idx = 0;
        for i in 0..26 {
            if f[i] == ma {
                idx = i;
            }
        }
        let b = n - ma + 1;
        if ma <= b {
            puts!("{}\n", intersperse(&f));
            continue;
        }
        let q = ma / b;
        let r = ma % b;
        let mut s = String::new();
        let mut del = vec![];
        for i in 0..26 {
            if i != idx {
                for _ in 0..f[i] {
                    del.push((b'a' + i as u8) as char);
                }
            }
        }
        assert_eq!(del.len(), b - 1);
        let mut lens = vec![];
        let mut aux = vec![];
        for (v, l) in [(q + 1, r), (q, b - r)] {
            if v % 2 == 0 {
                for _ in 0..l / 2 {
                    lens.push(v - 1);
                    lens.push(v + 1);
                }
                if l % 2 == 1 {
                    aux.push(v);
                }
            } else {
                for _ in 0..l {
                    lens.push(v);
                }
            }
        }
        for a in aux {
            lens.push(a);
        }
        assert_eq!(lens.len(), b);
        for i in 0..b {
            let l = lens[i];
            for _ in 0..l {
                s.push((b'a' + idx as u8) as char);
            }
            if i < b - 1 {
                s.push(del.pop().unwrap());
            }
        }
        puts!("{s}\n");
    }
}
