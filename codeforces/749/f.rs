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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn dfs(l: usize, r: usize, k: usize, col: &mut [Vec<i32>], c: i32) {
    let len = r - l;
    if len <= 1 {
        return;
    }
    for i in 0..k {
        let lo = l + i * len / k;
        let hi = l + (i + 1) * len / k;
        dfs(lo, hi, k, col, c + 1);
    }
    for i in 0..k - 1 {
        let lo = l + i * len / k;
        let mid = l + (i + 1) * len / k;
        for j in i + 1..k {
            let to = l + j * len / k;
            let hi = l + (j + 1) * len / k;
            for a in lo..mid {
                for b in to..hi {
                    col[a][b] = c;
                }
            }
        }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input!(n: usize, k: usize);
    let mut col = vec![vec![0; n]; n];
    dfs(0, n, k, &mut col, 1);
    let mut ma = 0;
    for i in 0..n {
        for j in 0..n {
            ma.chmax(col[i][j]);
        }
    }
    puts!("{}\n", ma);
    let mut ans = vec![];
    for i in 0..n {
        for j in i + 1..n {
            ans.push(col[i][j]);
        }
    }
    putvec!(ans);
}
