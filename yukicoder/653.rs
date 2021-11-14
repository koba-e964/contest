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
    input!(s: chars);
    let mut c = 0;
    let n = s.len();
    let mut sing = false;
    for i in (0..n).rev() {
        c += (s[i] as u8 - b'0') as i32;
        match c {
            6 | 7 if i + 1 != n => {
                c = 0;
                sing = true;
            }
            2 | 3 | 4 if !sing => c = -1,
            0 if i == 0 => {}
            _ => {
                println!("No");
                return;
            }
        }
    }
    println!("{}", if c == 0 { "Yes" } else { "No" });
}
