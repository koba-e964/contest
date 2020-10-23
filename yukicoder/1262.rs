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

fn main() {
    input! {
        n: usize,
        a: [usize],
    }
    let n = n + 1;
    // Problem: find \sum_{x: a} \sum_{j = 1}^x (gcd(x, j) - 1)
    let mut phi = vec![1i64; n];
    let mut fac = vec![0; n];
    for i in 1..n {
        fac[i] = i;
    }
    for i in 2..n {
        if fac[i] != i {
            continue;
        }
        for j in 2..(n - 1) / i + 1 {
            fac[i * j] = i;
        }
    }
    for i in 1..n {
        let mut v = i;
        let mut a = 1;
        while v > 1 {
            let p = fac[v];
            let mut e = 0;
            while v % p == 0 {
                e += 1;
                v /= p;
            }
            a *= p - 1;
            for _ in 0..e - 1 {
                a *= p;
            }
        }
        phi[i] = a as i64;
    }
    // \sum_[j = 1}^x gcd(x, i) = \sum_{g | x} g * phi(x / g)
    let mut vals = vec![0; n];
    for g in 1..n {
        for j in 1..(n - 1) / g + 1 {
            vals[g * j] += g as i64 * phi[j];
        }
    }
    let mut tot = 0;
    // (RHS) = \sum {x: a} vals[x] - x
    for a in a {
        tot += vals[a] - a as i64;
    }
    println!("{}", tot);
}
