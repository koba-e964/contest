#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut even = 0;
    let mut odd = 0;
    for &(x, y) in xy.iter() {
        if (x + y) % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    if even > 0 && odd > 0 {
        puts!("-1\n");
        return;
    }
    const W: usize = 35;
    let mut gadget: Vec<i64> = vec![];
    let mut ox = 0;
    let oy = 0;
    for i in 0 .. W {
        gadget.push(1 << i);
        // Assume all arms are R
        ox += 1 << i;
    }
    if even > 0 {
        gadget.push(1);
        ox += 1;
    }
    puts!("{}\n", gadget.len());
    for i in 0 .. gadget.len() {
        puts!("{}{}", if i == 0 { "" } else { " " }, gadget[i]);
    }
    puts!("\n");
    for i in 0 .. n {
        let (dx, dy) = (xy[i].0 - ox, xy[i].1 - oy);
        let (du, dv) = ((dy - dx) / 2, (-dy - dx) / 2);
        //puts!("{:?} {:?}", (dx, dy), (du, dv));
        let mut sol = vec![];
        for i in 0 .. W {
            let c = match (du & 1 << i, dv & 1 << i) {
                (0, 0) => 'R',
                (0, _) => 'D',
                (_, 0) => 'U',
                (_, _) => 'L',
            };
            sol.push(c);
        }
        if even > 0 { sol.push('R'); }
        puts!("{}\n", sol.into_iter().collect::<String>());
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
