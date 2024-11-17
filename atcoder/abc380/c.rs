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

// Tags: clustering, dedup
fn main() {
    input! {
        n: usize, k: usize,
        s: chars,
    }
    let mut con = vec![];
    let mut prev = 0;
    for i in 0..n {
        if s[i] == '1' {
            if i > 0 && s[i - 1] == '0' {
                prev = i;
            }
        } else {
            if i >= prev + 1 {
                con.push((prev, i));
            }
            prev = i + 1;
        }
    }
    if s[n - 1] == '1' {
        con.push((prev, n));
    }
    let mut s = s;
    let (_, y) = con[k - 2];
    let (z, w) = con[k - 1];
    for i in z..w {
        s.swap(i - z + y, i);
    }
    println!("{}", s.iter().collect::<String>());
}
