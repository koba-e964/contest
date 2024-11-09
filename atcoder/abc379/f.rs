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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        h: [i64; n],
        lr: [(usize1, usize1); q],
    }
    let mut qs = vec![vec![]; n];
    for i in 0..q {
        let (l, r) = lr[i];
        qs[l].push((r, i));
    }
    let mut st: Vec<usize> = vec![];
    let mut ans = vec![0; q];
    for i in (0..n).rev() {
        for &(r, idx) in qs[i].iter() {
            let from = match st.binary_search_by(|x| r.cmp(x)) {
                Ok(x) => x,
                Err(x) => x,
            };
            ans[idx] = from;
        }
        while let Some(v) = st.pop() {
            if h[i] < h[v] {
                st.push(v);
                break;
            }
        }
        st.push(i);
    }
    for a in ans {
        puts!("{a}\n");
    }
}
