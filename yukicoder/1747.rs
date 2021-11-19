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

fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    let mut d = 2;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    input!(s: chars);
    let n = s.len();
    let mut ans = 0;
    for bits in 0..1 << (n - 1) {
        let mut x = 0;
        let mut c = (s[0] as u8 - b'0') as i64;
        for i in 1..n {
            if (bits & 1 << (i - 1)) != 0 {
                x += c;
                c = 0;
            }
            c = 10 * c + (s[i] as u8 - b'0') as i64;
        }
        x += c;
        if is_prime(x) { ans += 1; }
    }
    println!("{}", ans);
}
