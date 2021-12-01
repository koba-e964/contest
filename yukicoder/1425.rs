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

// https://yukicoder.me/problems/no/1425 (3)
// i と i + 1 の 2 種類があれば [0, i + 1) のソートができる。すでにソートされていれば 0 個でよい。よって 1 個で可能かどうか判定すれば良い。
// i 1 個で十分なのは、ある j が存在して、a[0] <= .. <= a[j - 1], a[j] <= .. <= a[i - 1], a[i - 1] <= a[0], i < n であれば a[j - 1] <= a[i] が成立する場合である。特に a[k] > a[k + 1] を満たす k はちょうど 1 個でなければならない。
// Tags: answer-is-essentially-small
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut inv = vec![];
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            inv.push(i);
        }
    }
    if inv.is_empty() {
        println!("0");
        return;
    }
    if inv.len() >= 2 {
        println!("2");
        return;
    }
    let mut ans = 2;
    let j = inv[0];
    for i in j + 1..n + 1 {
        if a[i - 1] <= a[0] && (i == n || a[j] <= a[i]) {
            ans = 1;
        }
    }
    println!("{}", ans);
}
