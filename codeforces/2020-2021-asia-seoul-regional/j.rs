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

fn find_basis_gf2(a: &[Vec<i32>]) -> Vec<(Vec<i32>, Vec<i32>)> {
    let n = a.len();
    let m = a[0].len();
    let mut basis: Vec<(Vec<_>, Vec<_>)> = vec![];
    for i in 0..n {
        let mut cur = a[i].to_vec();
        let mut real = vec![0; n];
        real[i] = 1;
        for &(ref b, ref br) in &basis {
            let mut xor = cur.clone();
            for j in 0..m {
                xor[j] ^= b[j];
            }
            if xor < cur {
                cur = xor;
                for j in 0..n {
                    real[j] ^= br[j];
                }
            }
        }
        if cur.iter().any(|&x| x != 0) {
            basis.push((cur, real));
        }
    }
    basis
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
        a: [[i32; n]; n],
    }
    let basis = find_basis_gf2(&a);
    let mut res = vec![vec![]; n];
    for i in 0..n {
        let mut cur = vec![0; n];
        cur[i] = 1;
        let mut real = vec![0; n];
        for &(ref b, ref br) in &basis {
            let mut xor = cur.clone();
            for j in 0..n {
                xor[j] ^= b[j];
            }
            if xor < cur {
                cur = xor;
                for j in 0..n {
                    real[j] ^= br[j];
                }
            }
        }
        if cur.iter().all(|&x| x == 0) {
            let mut ans = vec![];
            for j in 0..n {
                if real[j] == 1 {
                    ans.push(j + 1);
                }
            }
            res[i] = ans;
        } else {
            puts!("-1\n");
            return;
        }
    }
    for i in 0..n {
        putvec!(res[i]);
    }
}
