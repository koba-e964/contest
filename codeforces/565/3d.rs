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
        n: usize,
        a: [usize; 2 * n],
    }
    const W: usize = 2750132;
    let mut pr = vec![true; W];
    let mut fac = vec![0; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        fac[i] = i;
        let mut j = 2 * i;
        while j < W {
            pr[j] = false;
            if fac[j] == 0 { fac[j] = i; }
            j += i;
        }
    }
    let primes: Vec<_> = (2..W).filter(|&x| pr[x]).collect();
    let mut pidx = vec![0; W];
    for i in 0..primes.len() {
        pidx[primes[i]] = i;
    }
    let mut a = a;
    a.sort(); a.reverse();
    let mut hm = HashMap::new();
    for &a in &a {
        *hm.entry(a).or_insert(0) += 1;
    }
    let mut ans = vec![];
    for &a in &a {
        if hm[&a] == 0 { continue; }
        if !pr[a] {
            let maxdiv = a / fac[a];
            ans.push(a);
            *hm.get_mut(&a).unwrap() -= 1;
            *hm.get_mut(&maxdiv).unwrap() -= 1;
        }
    }
    for &a in &a {
        if hm[&a] == 0 { continue; }
        assert!(pr[a]);
        let idx = pidx[a] + 1;
        ans.push(idx);
        *hm.get_mut(&a).unwrap() -= 1;
        *hm.get_mut(&idx).unwrap() -= 1;
    }
    for i in 0..n {
        puts!("{}{}", ans[i], if i + 1 == n { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
