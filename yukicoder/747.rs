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
        n: chars,
        k: chars,
    }
    let mut x = 0;
    for &c in &n {
        x = 10 * x + (c as u8 - b'0') as i64;
        x %= 6;
    }
    let mut prod = 1;
    let mut cur = x;
    for &c in k.iter().rev() {
        let u = (c as u8 - b'0') as i64;
        for _ in 0..u {
            prod = prod * cur % 6;
        }
        let mut y = 1;
        for _ in 0..10 {
            y = y * cur % 6;
        }
        cur = y;
    }
    println!("{}", [4, 2, 8, 5, 7, 1][prod as usize]);
}
