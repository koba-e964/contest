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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        s: chars,
        targ: i64,
    }
    let mut dec = 0i64;
    let mut mask = 0i64;
    let n = s.len();
    for i in 0..n {
        if s[i] == '?' {
            mask |= 1 << (n - 1 - i);
        }
        if s[i] == '1' {
            dec |= 1 << (n - 1 - i);
        }
    }
    if dec > targ {
        println!("-1");
        return;
    }
    let mut cur = dec;
    for i in (0..60).rev() {
        if (mask & 1 << i) != 0 && (cur | 1 << i) <= targ {
            cur |= 1 << i;
        }
    }
    println!("{}", cur);
}
