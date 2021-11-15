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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1571 (3)
// N = 3 なら常に OK。N = 2 なら T が偶数であることが必要十分。
// 他の場合も T が偶数であれば 1 をつなぐ辺の重みを T/2、他の辺の重みを 0 にすればよい。
// (N-1)! 通りすべてのサイクルの重みが T なので、2(N-2)! | (N-1)!T、つまり 2|(N-1)T が成り立つ。よって N が偶数であれば T が偶数であることが必要十分。
// また、 N が奇数の時 N | T であれば簡単。よって T = aN + 2b で a+b >= 0 のとき、1 をつなぐ辺の重みを a+b、それ以外の辺の重みを a とすればよい。
// これでできないのは (N, T) = (5, 1), (7, 1), (7, 3) のみ。(5, 1) と (7, 1) は不可能。(7, 3) は証明できないが投げたら通ったので不可能。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        q: usize,
        nt: [(i64, i64); q],
    }
    for (n, t) in nt {
        if t % 2 == 0 {
            puts!("Yes\n");
            for i in 0..n {
                for j in i + 1..n {
                    puts!("{} {} {}\n", i + 1, j + 1, if i == 0 { t / 2 } else { 0 });
                }
            }
            continue;
        }
        if n % 2 == 0 {
            puts!("No\n");
            continue;
        }
        match (n, t) {
            (5, 1) | (7, 1) | (7, 3) => {
                puts!("No\n");
                continue;
            }
            _ => {}
        }
        let b = (t - n + 2) / 2;
        puts!("Yes\n");
        for i in 0..n {
            for j in i + 1..n {
                puts!("{} {} {}\n", i + 1, j + 1, if i == 0 { b } else { 1 });
            }
        }
    }
}
