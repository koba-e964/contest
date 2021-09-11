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

// Tags: constant-factor-optimization, low-memory
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 1_000_001;
    let mut seen = vec![0; W];
    let mut g = vec![0; W];
    for i in 1..W {
        let mut mex = 0;
        while (seen[i] & 1 << mex) != 0 {
            mex += 1;
        }
        g[i] = mex;
        for j in 2..(W - 1) / i + 1 {
            seen[i * j] |= 1 << g[i];
        }
    }
    let mut ans = 0;
    for a in a {
        ans ^= g[a];
    }
    println!("{}", if ans == 0 { "black" } else { "white" });
}
