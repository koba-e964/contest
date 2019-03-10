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

fn tr(a: i32) -> i32 {
    if a <= 5 {
        return 0;
    }
    match a {
        6 => 1,
        7 => 1,
        8 => 2,
        9 => 3,
        _ => panic!(),
    }
}

fn calc(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut mi = 100;
    for i in 0..n {
        if a[i] != 5 { continue; }
        let mut tmp = 1;
        let mut ma = 0;
        for j in 0..i {
            ma = max(ma, a[j]);
        }
        tmp += tr(ma);
        ma = 0;
        for j in i + 1..n {
            ma = max(ma, a[j]);
        }
        tmp += tr(ma);
        mi = min(mi, tmp);
    }
    mi
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }
    let mut zero = 0;
    let mut five = 0;
    let mut ma = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 { zero += 1; }
            if a[i][j] == 5 { five += 1; }
            ma = max(ma, a[i][j]);
        }
    }
    if zero == h * w {
        puts!("Yes 0\n");
        return;
    }
    if five == 0 {
        puts!("No\n");
        return;
    }
    if h == 1 || w == 1 {
        let mut seq = vec![0; h * w];
        for i in 0..h {
            for j in 0..w {
                seq[i * w + j] = a[i][j];
            }
        }
        puts!("Yes {}\n", calc(seq));
        return;
    }
    let cnt;
    if ma <= 5 {
        cnt = 1;
    } else {
        cnt = match ma {
            6 => 1,
            7 => 1,
            8 => 2,
            9 => 3,
            _ => panic!(),
        } + 1;
    }
    puts!("Yes {}\n", cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
