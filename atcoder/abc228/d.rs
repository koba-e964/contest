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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        q: usize,
        tx: [(i32, i64); q],
    }
    let n = 1 << 20;
    let m = 1 << 10;
    let mut tbl = vec![-1; n];
    let mut vac = vec![m; m];
    for (t, x) in tx {
        if t == 2 {
            let x = (x % n as i64) as usize;
            puts!("{}\n", tbl[x]);
        } else {
            let mut h = (x % n as i64) as usize;
            while tbl[h] != -1 {
                if h % m == 0 && vac[h / m] == 0 {
                    h = (h + m) % n;
                } else {
                    h = (h + 1) % n;
                }
            }
            tbl[h] = x;
            vac[h / m] -= 1;
        }
    }
}
