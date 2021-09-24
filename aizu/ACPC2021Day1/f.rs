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
        t: chars,
    }
    let mut u = vec![];
    for c in s {
        if let Some(&l) = u.last() {
            if l == c as u8 {
                u.push(b'0' ^ b'1' ^ l as u8);
            }
        }
        u.push(c as u8);
    }
    let mut pos = 0;
    for c in u {
        if pos < t.len() && c == t[pos] as u8 {
            pos += 1;
        }
    }
    println!("{}", if pos == t.len() { "Yes" } else { "No" });
}
