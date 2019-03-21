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

fn rec(a: usize, n: usize, mask: usize) -> Vec<usize> {
    assert_ne!(a, 0);
    let lsb = a & a.wrapping_neg();
    let v = n - mask.count_ones() as usize;
    if lsb == a {
        let mut ret = vec![0; 1 << (n - v)];
        let mut biasinv = 0;
        let mut bits = 0;
        let mut invmap = vec![0; 1 << (n - v)];
        for pos in 0..1 << n {
            if (mask & pos) != pos { continue; }
            invmap[bits] = pos;
            if pos == a {
                biasinv = bits;
            }
            bits += 1;
        }
        for bits in 0..1 << (n - v) {
            let b = (bits + biasinv) & ((1 << (n - v)) - 1);
            ret[bits] = invmap[b ^ (b >> 1) ^ biasinv ^ (biasinv >> 1)];
        }
        //eprintln!("a = {}, v = {}, ret = {:?}", a, v, ret);
        assert_eq!(ret[0] ^ ret[(1 << (n - v)) - 1], a);
        return ret;
    }
    let rest = a ^ lsb;
    let lsb2 = rest & rest.wrapping_neg();
    let sub = rec(lsb2, n, mask ^ lsb);
    let sub2 = rec(rest ^ lsb2, n, mask ^ lsb);
    let mut ret = vec![0; 1 << (n - v)];
    for i in 0..1 << (n - v - 1) {
        ret[i] = sub[i];
    }
    for i in 0..1 << (n - v - 1) {
        ret[i + (1 << (n - v - 1))] = sub2[i] ^ lsb ^ lsb2;
    }
    ret
}

fn calc(a: usize, n: usize) -> Option<Vec<usize>> {
    if a.count_ones() % 2 == 0 { return None; }
    Some(rec(a, n, (1 << n) - 1))
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let p = calc(a ^ b, n);
    match p {
        None => {
            puts!("NO\n");
        }
        Some(x) => {
            puts!("YES\n");
            for i in 0..1 << n {
                puts!("{}{}", x[i] ^ a, if i + 1 == 1 << n { "\n" } else { " " });
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
