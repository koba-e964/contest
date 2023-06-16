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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn encode(a: Vec<usize>) -> Vec<bool> {
    let mut res = vec![];
    for i in 0..a.len() {
        if i > 0 {
            res.push(true);
        }
        res.extend_from_slice(&vec![false; a[i]]);
    }
    res
}

// https://yukicoder.me/problems/no/1818 (3.5)
// 整数列 A を、0 を A[1] 個、　1 を 1 個、0 を A[2] 個、...、0 を A[N] 個ならべたものとして 0 と 1 の列にエンコードする。
// この時、操作は 0 や 1 を挿入・削除、 0 と 1 の書き換えに対応する。
// A も B も長さが 6 * 10^3 程度であるため、編集距離を求めれば良い。
// Tags: operations, bijections, finding-equivalent-notions
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let a = encode(a);
    let b = encode(b);
    let n = a.len();
    let m = b.len();
    const INF: i32 = 1 << 28;
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            if i + j == 0 { continue; }
            let mut me = INF;
            if i > 0 {
                me.chmin(dp[i - 1][j] + 1);
            }
            if j > 0 {
                me.chmin(dp[i][j - 1] + 1);
            }
            if i > 0 && j > 0 {
                me.chmin(dp[i - 1][j - 1] + if a[i - 1] == b[j - 1] { 0 } else { 1 });
            }
            dp[i][j] = me;
        }
    }
    println!("{}", dp[n][m]);
}
