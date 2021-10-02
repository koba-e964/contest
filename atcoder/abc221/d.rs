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
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut coo = vec![];
    for &(a, b) in &ab {
        coo.push(a);
        coo.push(a + b);
    }
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut imos = vec![0; m + 1];
    for &(a, b) in &ab {
        let l = coo.binary_search(&a).unwrap();
        let r = coo.binary_search(&(a + b)).unwrap();
        imos[l] += 1;
        imos[r] -= 1;
    }
    for i in 1..m + 1 {
        imos[i] += imos[i - 1];
    }
    let mut ans = vec![0; n + 1];
    for i in 0..m - 1 {
        ans[imos[i] as usize] += coo[i + 1] - coo[i];
    }
    putvec!(ans[1..n + 1]);
}
