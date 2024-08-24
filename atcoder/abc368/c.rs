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
        n: usize,
        h: [i64; n],
    }
    let mut t = 0;
    for mut h in h {
        while h > 0 && t % 3 != 0 {
            if t % 3 == 2 {
                h -= 3;
            } else {
                h -= 1;
            }
            t += 1;
        }
        let q = h / 5;
        h -= q * 5;
        t += q * 3;
        while h > 0 {
            if t % 3 == 2 {
                h -= 3;
            } else {
                h -= 1;
            }
            t += 1;
        }
    }
    println!("{}", t);
}
