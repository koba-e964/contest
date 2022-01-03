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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(a: &[i8]) -> i32 {
    let n = a.len();
    let mut sum = 0;
    for i in 0..n {
        if a[i] == 1 {
            sum += 1 << (i % 2);
        }
    }
    sum % 3
}

// https://yukicoder.me/problems/no/1613 (3)
// 列ごとに見たゲームの Nim 和であることは明らか。実験すると、列の配置を二進数とみなした場合、
// x の Grundy 数は、x >= 8 であれば 2^p | x である最大の p であることがわかる。
// -> 実験コードが間違っていた。再度実験した結果 Grundy 数は x % 3 であることがわかった。
// Tags: nim, game
fn main() {
    input! {
        h: usize, w: usize,
        b: [chars; h],
    }
    let mut g = 0;
    for i in 0..w {
        let mut v = vec![0; h];
        for j in 0..h {
            v[j] = if b[j][i] == 'o' { 1 } else { 0 };
        }
        g ^= calc(&v);
    }
    println!("{}", if g == 0 { "Second" } else { "First" });
}
