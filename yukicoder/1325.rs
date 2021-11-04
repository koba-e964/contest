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

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    if n == 1 {
        println!("{}", a[0] % MOD);
        return;
    }
    // 2^{n-1-i} * \sum_{j <= i} C(i, j) (j + 1) = (i+2)2^{n-2}
    let mut s = 0;
    for i in 0..n {
        s += (i + 2) as i64 * a[i];
        s %= MOD;
    }
    for _ in 0..n - 2 {
        s = s * 2 % MOD;
    }
    println!("{}", s);
}
