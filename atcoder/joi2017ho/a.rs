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
        n: usize, q: usize, s: i64, t: i64,
        a: [i64; n + 1],
        lrx: [(usize1, usize, i64); q],
    }
    let mut diff = vec![0; n];
    let mut val = vec![0; n];
    let mut sum = 0;
    let f = |x: i64| if x >= 0 { -s * x } else { -t * x };
    for i in 0..n {
        diff[i] = a[i + 1] - a[i];
        val[i] = f(diff[i]);
        sum += val[i];
    }
    for (l, r, x) in lrx {
        sum += f(diff[l] + x) - f(diff[l]);
        diff[l] += x;
        if r < n {
            sum += f(diff[r] - x) - f(diff[r]);
            diff[r] -= x;
        }
        puts!("{}\n", sum);
    }
}
