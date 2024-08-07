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

fn lis(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut mi = vec![!0; n + 1];
    mi[0] = 0;
    let mut dp = vec![0; n];
    for i in 0..n {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if mi[mid] < a[i] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        dp[i] = pass + 1;
        mi[dp[i]] = mi[dp[i]].min(a[i]);
    }
    dp
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        t: usize,
        a: [[usize]; t],
    }
    for a in a {
        let n = a.len();
        let mut b = a.clone();
        b.reverse();
        for i in 0..n {
            b[i] = !b[i];
        }
        let al = lis(&a);
        let mut bl = lis(&b);
        bl.reverse();
        let ma = al.iter().max().copied().unwrap();
        let mut ans = vec![];
        for i in 0..n {
            if al[i] + bl[i] - 1 == ma {
                ans.push(i + 1);
            }
        }
        puts!("{}\n", ans.len());
        putvec!(ans);
    }
}
