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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        a: [usize1; n],
    }
    let mut occ = vec![vec![]; n];
    for i in 0..n {
        occ[a[i]].push(i);
    }
    let mut has = false;
    for i in 0..n {
        if occ[i].is_empty() { continue; }
        let l = occ[i][0];
        let r = occ[i][occ[i].len() - 1];
        if r - l + 1 != occ[i].len() {
            if has || (l, r) != (0, n - 1) {
                println!("-1");
                return;
            }
            let mut p = 0;
            for j in 0..occ[i].len() {
                if j != occ[i][j] {
                    p = j;
                    break;
                }
            }
            for j in p..occ[i].len() {
                if occ[i].len() - j != n - occ[i][j] {
                    println!("-1");
                    return;
                }
            }
            has = true;
        }
    }
    println!("{}", if has { 1 } else { 0 });
}
