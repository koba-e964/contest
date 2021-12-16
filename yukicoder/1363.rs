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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1363 (3)
// Z君は Grundy 数を 0 以外にできたら勝ち、C君は Grundy 数を 0 にできたら勝ち。
// 色 0 のベルをちょうど b 個使用する場合、新しい色の紙に書く整数を調整して実現できる Grundy 数は b または 0 以上 b/2 未満の整数である。(紙に書く整数を f としたとき Grundy 数は b % (f + 1) なので。)
// 実際は色 0 のベルは多く持っている方がよいので、Grundy 数として「0 以上 b/2 未満の整数」を選ぶ意味はない。
// 最初の盤面の Grundy 数を g とすると、C君の番の直前に Y >= X + g であることが C君が勝てることと同値。
// (Y >= X + g => C君が勝てる): 初手で g 個のベルを置き、紙に g を書く。その後は Z君が実現した Grundy 数と同じ個数のベルを置いていけば、置いた直後の Grundy 数は常に 0。
// (Y < X + g => C君が負ける): (Y, X) の上の帰納法 (辞書式順序)。Z君の番の直前に Grundy 数が 0 以外であれば、パスをすれば良い。0 の場合、Y 君が g 個以上のベルを置いたので、今は Y-g 個以下のベルしか持っていない。Y - g < X であるため、Z君が 1 個置けば (Y, X) がより小さい状態で Y < X + 1 が満たされている。
// Tags: game, nim
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(rearr: [([[i64; 2]], i64, i64)]);
    for (s, t, u) in rearr {
        let k = s.len();
        let mut a = vec![];
        for i in 0..k {
            for j in 0..2 {
                a.push(s[i][j]);
            }
        }
        a.push(t);
        a.push(u);
        let x = a[0];
        let y = a[1];
        let p = a.split_off(k + 2);
        let a = a.split_off(2);
        let mut g = 0;
        for i in 0..k {
            g ^= a[i] % (p[i] + 1);
        }
        puts!("{}\n", if y >= x + g { "C" } else { "Z" });
    }
}
