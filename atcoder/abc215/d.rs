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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }
    const W: usize = 100_100;
    let mut fac = vec![0; W];
    for i in 2..W {
        if fac[i] != 0 {
            continue;
        }
        fac[i] = i;
        for j in 2..(W - 1) / i + 1 {
            fac[i * j] = i;
        }
    }
    let mut used = vec![];
    for mut a in a {
        while a != 1 {
            let p = fac[a];
            while a % p == 0 {
                a /= p;
            }
            used.push(p);
        }
    }
    used.sort_unstable(); used.dedup();
    let mut ban = vec![0; m + 1];
    for u in used {
        for i in 1..m / u + 1 {
            ban[u * i] = 1;
        }
    }
    let ans: Vec<_> = (1..m + 1).filter(|&x| ban[x] == 0).collect();
    puts!("{}\n", ans.len());
    for a in ans {
        puts!("{}\n", a);
    }
}
