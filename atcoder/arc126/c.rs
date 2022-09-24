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

// Tags: sum-of-reciprocals
fn main() {
    input! {
        n: usize, k: i64,
        a: [usize; n],
    }
    const W: usize = 600_001;
    let mut f = vec![0i64; W];
    for a in a {
        f[a] += 1;
    }
    let mut acc = vec![0; W];
    let mut bcc = vec![0; W];
    for i in 1..W {
        acc[i] = acc[i - 1] + f[i];
        bcc[i] = bcc[i - 1] + f[i] * i as i64;
    }
    let mut g = 1;
    for i in 2..W {
        let mut tot = 0;
        for j in 1..(W - 1) / i + 1 {
            tot += (acc[i * j] - acc[i * j - i]) * (i * j) as i64 - (bcc[i * j] - bcc[i * j - i]);
        }
        if tot <= k {
            g = i as i64;
        }
    }
    if acc[W - 1] * (W - 1) as i64 - bcc[W - 1] <= k {
        let rem = k - (acc[W - 1] * (W - 1) as i64 - bcc[W - 1]);
        g = (W - 1) as i64 + rem / n as i64;
    }
    println!("{}", g);
}
