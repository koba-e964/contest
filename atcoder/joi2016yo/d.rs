use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, t: i64, q: usize,
        ad: [(i64, i8); n],
        x: [usize1; q],
    }
    let mut imp = vec![false; n];
    for x in x {
        imp[x] = true;
    }
    let mut seq = vec![];
    let mut last = 1;
    let mut cur = vec![];
    for i in 0..n {
        let (a, d) = ad[i];
        if last != d {
            seq.push(cur);
            last = d;
            cur = vec![];
        }
        cur.push((a, imp[i]));
    }
    seq.push(cur);
    if seq.len() % 2 != 0 {
        seq.push(vec![]);
    }
    for i in 0..seq.len() / 2 {
        let x = &seq[2 * i];
        let y = &seq[2 * i + 1];
        if x.is_empty() {
            for &(y, disp) in y {
                if disp {
                    puts!("{}\n", y - t);
                }
            }
            continue;
        }
        if y.is_empty() {
            for &(x, disp) in x {
                if disp {
                    puts!("{}\n", x + t);
                }
            }
            continue;
        }
        let meet = (x[x.len() - 1].0 + y[0].0) / 2;
        for &(x, disp) in x {
            if disp {
                puts!("{}\n", min(x + t, meet));
            }
        }
        for &(y, disp) in y {
            if disp {
                puts!("{}\n", max(y - t, meet));
            }
        }
    }
}
