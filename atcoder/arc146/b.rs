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

// 2^30 が可能なのに注意。
fn main() {
    input! {
        n: usize, m: i64, k: usize,
        a: [i64; n],
    }
    let mut ans = 0;
    for i in (0..31).rev() {
        let new = ans | 1 << i;
        let mut nec = vec![0; n];
        for i in 0..n {
            let absent = !a[i] & new;
            if absent != 0 {
                let x = absent.leading_zeros();
                let y = (1 << (63 - x)) - 1;
                nec[i] = ((a[i] & !y) | new) - a[i];
            }
        }
        nec.sort();
        if nec[..k].iter().sum::<i64>() <= m {
            ans = new;
        }
    }
    println!("{}", ans);
}
