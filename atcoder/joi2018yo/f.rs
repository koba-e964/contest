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

fn main() {
    input! {
        n: usize, k: usize, l: i64,
        a: [i64; n],
    }
    let mut fail = 0;
    let mut pass = n as i64;
    while pass - fail > 1 {
        let mid = (fail + pass) / 2;
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + if a[i] <= mid { 1 } else { 0 };
        }
        let mut pos = k;
        let mut tmp = 0;
        for i in 0..n {
            while pos <= n && acc[pos] - acc[i] < k {
                pos += 1;
            }
            tmp += (n + 1 - pos) as i64;
        }
        if tmp >= l {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
