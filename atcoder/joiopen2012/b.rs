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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// The author solved this with hint: https://kmjp.hatenablog.jp/entry/2012/09/21/0930
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut xyi = vec![];
    for i in 0..n {
        xyi.push((xy[i], i));
    }
    xyi.sort();
    let (x0, y0) = xyi[0].0;
    let (x1, y1) = xyi[n - 1].0;
    let mut fst = vec![xyi[0].1];
    let mut snd = vec![];
    let mut pos = false;
    let mut neg = false;
    for i in 1..n - 1 {
        let ((x, y), _idx) = xyi[i];
        let o = (x1 - x0) * y - (y1 - y0) * x;
        let o0 = (x1 - x0) * y0 - (y1 - y0) * x0;
        if o > o0 {
            pos = true;
        }
        if o < o0 {
            neg = true;
        }
    }
    if !pos && !neg {
        puts!("0\n");
        return;
    }
    for i in 1..n - 1 {
        let ((x, y), idx) = xyi[i];
        let o = (x1 - x0) * y - (y1 - y0) * x;
        let o0 = (x1 - x0) * y0 - (y1 - y0) * x0;
        if o > o0 || (o == o0 && neg) {
            fst.push(idx);
        } else {
            snd.push(idx);
        }
    }
    fst.push(xyi[n - 1].1);
    snd.reverse();
    fst.extend(&snd);
    for i in 0..n {
        puts!("{}\n", fst[i] + 1);
    }
}
