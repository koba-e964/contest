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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1377 (3)
// x=i と x=i+1 をこの順番で行うと (a[i], a[i + n]) が (0, 0) -> (0, 0), (0, 1) -> (1, 0), (1, 0) -> (1, 1), (1, 1) -> (0, 1) となる。逆順だと (0, 0) -> (0, 0), (0, 1) -> (1, 1), (1, 0) -> (0, 1), (1, 1) -> (1, 0) となる。これで全ての遷移が尽くされたので、(0, 0) とそれ以外の間の遷移がなければ 2N 回以内に可能。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        s: chars,
        t: chars,
    }
    let tbl = [
        [0, -1, -1, -1],
        [-1, 0, 1, 2],
        [-1, 2, 0, 1],
        [-1, 1, 2, 0],
    ];
    let get = |s: &[char], x: usize| -> usize {
        let a = s[x] as u8 - b'0';
        let b = s[n + x] as u8 - b'0';
        (a * 2 + b) as usize
    };
    let mut ans = vec![];
    for i in 0..n {
        let a = get(&s, i);
        let b = get(&t, i);
        let k = tbl[a][b];
        if k < 0 {
            puts!("-1\n");
            return;
        }
        if k > 0 {
            if k == 1 {
                ans.push(i);
                ans.push(i + 1);
            } else {
                ans.push(i + 1);
                ans.push(i);
            }
        }
    }
    puts!("{}\n", ans.len());
    for a in ans {
        puts!("{}\n", a);
    }
}
