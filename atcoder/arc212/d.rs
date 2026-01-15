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

// solved with hints
fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }
    let mut a = a;
    let mut x = vec![false; n];
    let mut sum = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            sum[i] += a[i][j];
        }
    }
    let mut cont = true;
    while cont {
        cont = false;
        if let Some(idx) = sum.iter().position(|&v| v < 0) {
            cont = true;
            x[idx] ^= true;
            for i in 0..n {
                sum[idx] -= a[idx][i] * 2;
                a[idx][i] *= -1;
                sum[i] -= a[i][idx] * 2;
                a[i][idx] *= -1;
            }
        }
    }
    for i in 0..n {
        print!("{}", if x[i] { "Y" } else { "X" });
    }
    println!();
}
