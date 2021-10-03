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

// Tags: high-elements, stack
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        tlr: [(i32, usize1, usize); q],
    }
    let mut ans = vec![-1; q];
    let mut buc = vec![vec![]; n];
    for i in 0..q {
        let (_, l, r) = tlr[i];
        buc[l].push((r, i));
    }
    let mut st: Vec<(i64, usize)> = vec![];
    for i in (0..n).rev() {
        while let Some(v) = st.pop() {
            if v.0 > a[i] {
                st.push(v);
                break;
            }
        }
        st.push((a[i], i));
        let m = st.len();
        for &(r, idx) in &buc[i] {
            let mut pass = 0;
            let mut fail = m + 1;
            while fail - pass > 1 {
                let mid = (fail + pass) / 2;
                if st[m - mid].1 < r {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            ans[idx] = pass as i32;
        }
    }
    for i in 0..q {
        puts!("{}\n", ans[i]);
    }
}
