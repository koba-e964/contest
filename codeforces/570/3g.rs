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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        af: [[(usize1, i32)]],
    }
    for af in &af {
        let n = af.len();
        let mut freq = vec![[0; 2]; n + 1];
        for &(a, f) in af {
            freq[a][f as usize] += 1;
        }
        let mut freqs = vec![];
        for i in 0..n + 1 {
            freqs.push((freq[i][0] + freq[i][1], freq[i][1]));
        }
        freqs.sort();
        freqs.reverse();
        let mut using = vec![];
        let mut tot0 = 0;
        let mut tot1 = 0;
        let mut f = freqs[0].0;
        for i in 0..n + 1 {
            f = min(f, freqs[i].0);
            using.push(f);
            tot0 += f;
            f -= 1;
            if f <= 0 {
                break;
            }
        }
        let mut pos = 0;
        let mut que = BinaryHeap::new();
        using.reverse();
        while let Some(len) = using.pop() {
            while pos < n + 1 {
                if freqs[pos].0 >= len {
                    que.push(freqs[pos].1);
                    pos += 1;
                } else {
                    break;
                }
            }
            if let Some(onelen) = que.pop() {
                tot1 += min(len, onelen);
            }
        }
        puts!("{} {}\n", tot0, tot1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
