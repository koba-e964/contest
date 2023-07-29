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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/2249 (3)
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 10_000_001;
    let mut phi = vec![0i64; W];
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        phi[i] = i as i64;
    }
    for i in 2..W {
        if !pr[i] { continue; }
        phi[i] = i as i64 - 1;
        for j in 2..=(W - 1) / i {
            pr[i * j] = false;
            phi[i * j] /= i as i64;
            phi[i * j] *= i as i64 - 1;
        }
    }
    let mut acc = vec![0i64; W];
    for i in 2..W {
        acc[i] = acc[i - 1] + (i as i64 - 1) * 2 - phi[i];
    }
    for a in a {
        puts!("{}\n", acc[a]);
    }
}
