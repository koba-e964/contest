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

// https://yukicoder.me/problems/no/2359 (3)
// Tags: sqrt-decomposition
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        lrxy: [(usize, usize, usize, usize); n],
        a: [usize; m],
    }
    const W: usize = 100_001;
    const B: usize = 320;
    let mut imos = vec![vec![0i64; W]; B];
    let mut freq = vec![0i64; W];
    for (l, r, x, y) in lrxy {
        let loidx = (l + x - y - 1) / x;
        let hiidx = (r + x - y) / x;
        if x < B {
            let lo = loidx * x + y;
            let hi = hiidx * x + y;
            if lo < hi {
                if hi < W {
                    imos[x][hi] -= 1;
                }
                imos[x][lo] += 1;
            }
        } else {
            for i in loidx..hiidx {
                freq[y + i * x] += 1;
            }
        }
    }
    for i in 1..B {
        for j in 0..W - i {
            imos[i][j + i] += imos[i][j];
        }
        for j in 0..W {
            freq[j] += imos[i][j];
        }
    }
    for a in a {
        puts!("{}\n", freq[a]);
    }
}
