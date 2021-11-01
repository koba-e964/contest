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

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

// Find a generator of (Z/pZ)^\times
fn gen_zpz(p: i64) -> i64 {
    let mut v = p - 1;
    let mut f = 2;
    let mut fs = vec![];
    while v >= f * f {
        if v % f == 0 {
            fs.push(f);
            while v % f == 0 { v /= f; }
        }
        f += 1;
    }
    if v > 1 {
        fs.push(v);
    }
    let mut g = 2;
    loop {
        if fs.iter().all(|&x| powmod(g, (p - 1) / x, p) != 1) {
            return g;
        }
        g += 1;
    }
}

// Tags: generators-of-cyclic-groups
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        t: usize,
        vx: [(i64, i64); t],
    }
    for (v, x) in vx {
        let p = v * x + 1;
        let g = gen_zpz(p);
        let k = powmod(g, v, p);
        let mut ans = vec![];
        let mut c = 1;
        for _ in 0..x {
            ans.push(c);
            c = c * k % p;
        }
        ans.sort_unstable();
        putvec!(ans);
    }
}
