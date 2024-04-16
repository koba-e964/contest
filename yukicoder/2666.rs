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

// https://yukicoder.me/problems/no/2666 (3.5)
// XOR floor(A[i]/M) != 0 であれば初手で M の倍数取れば普通の Nim と同じように先手必勝。
// M = 3 のとき [6, 6, 3, 4] あたりは先手必勝らしい。
// 実験によると M が十分大の時は通常の Nim と同様に見え、通常時も XOR floor(A[i]/M) == 0 であれば
// XOR floor(A[i]%M) で決まっているように見える。
// Tags: nim, unusual-nim, decreasing-nim
fn main() {
    input! {
        n: usize, m: i64,
        a: [i64; n],
    }
    let mut gq = 0;
    let mut gr = 0;
    for &a in &a {
        gq ^= a / m;
        gr ^= a % m;
    }
    println!("{}", if gq == 0 && gr == 0 { "Bob" } else { "Alice "});
}
