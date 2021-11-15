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

// https://yukicoder.me/problems/no/1567 (3.5)
// k - P が異なる 5 個の整数の積で表せれば良い。
// ±1 の存在も考えると、3 個以上の異なる素因数を持つ、あるいは p^2q, p^4 以上であればよい。(p^3 はダメ)
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        plr: [(i32, i32, i32); t],
    }
    const W: usize = 400_100;
    let mut fac = vec![0; W];
    for i in 2..W {
        if fac[i] != 0 { continue; }
        fac[i] = i;
        for j in 2..(W - 1) / i + 1 {
            fac[i * j] = i;
        }
    }
    let mut acc = vec![0; W];
    for i in 2..W {
        let mut me = 0;
        let mut v = i;
        let mut f = vec![];
        while v > 1 {
            let p = fac[v];
            let mut e = 0;
            while v % p == 0 {
                v /= p;
                e += 1;
            }
            f.push(e);
        }
        if f.len() >= 3 {
            me = 1;
        }
        if f.len() == 2 && f[0] + f[1] >= 3 {
            me = 1;
        }
        if f.len() == 1 && f[0] >= 4 {
            me = 1;
        }
        acc[i] = acc[i - 1] + me;
    }
    let lower = |x: i32| {
        if x >= 0 {
            1 + acc[x as usize]
        } else {
            -acc[(-x - 1) as usize]
        }
    };
    for (p, l, r) in plr {
        puts!("{}\n", lower(r - p) - lower(l - p - 1));
    }
}
