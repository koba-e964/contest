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

fn two(a: &[i32]) -> Vec<i64> {
    let mut res = vec![0; a.len()];
    let mut c = 0i64;
    for i in 0..a.len() {
        if a[i] == 2 {
            res[i] = c;
        }
        if a[i] != 1 {
            c = 0;
        } else {
            c += 1;
        }
    }
    res
}

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut s0 = n as i64 * (n as i64 - 1) / 2;
    let mut s1 = 0;
    let mut s2 = 0;
    let mut c = 0i64;
    for i in 0..n {
        if a[i] == 0 {
            s0 -= c * (c - 1) / 2;
            c = 0;
        } else {
            c += 1;
        }
    }
    s0 -= c * (c - 1) / 2;
    c = 0;
    for i in 0..n {
        if a[i] != 1 {
            s1 += c * (c - 1) / 2;
            c = 0;
        } else {
            c += 1;
        }
    }
    s1 += c * (c - 1) / 2;
    let tmp0 = two(&a);
    let mut a = a;
    a.reverse();
    let mut tmp1 = two(&a);
    tmp1.reverse();
    for i in 0..n {
        s2 += (tmp0[i] + 1) * (tmp1[i] + 1) - 1;
    }
    println!("{} {} {}", s0, s1, s2);
}
