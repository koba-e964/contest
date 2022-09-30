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

// Tags: tree-counting, fps
fn main() {
    input! {
        n: usize,
        d: [i64; n],
    }
    let mut prod = 1;
    for i in 0..n { prod = prod * d[i] % MOD; }
    let dsum: i64 = d.iter().map(|&d| d - 1).sum();
    if dsum < n as i64 - 2 {
        println!("0");
        return;
    }
    for i in 0..n - 2 {
        prod = prod * ((dsum - i as i64) % MOD) % MOD;
    }
    println!("{}", prod);
}
