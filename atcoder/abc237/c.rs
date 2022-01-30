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
    if s.iter().all(|&c| c == 'a') {
        println!("Yes");
        return;
    }
    let n = s.len();
    let mut l = 0;
    let mut r = n;
    while s[l] == 'a' {
        l += 1;
    }
    while s[r - 1] == 'a' {
        r -= 1;
    }
    if l > n - r {
        println!("No");
        return;
    }
    let z = s[l..r].to_vec();
    let mut w = z.clone();
    w.reverse();
    println!("{}", if w == z { "Yes" } else { "No" });
}
