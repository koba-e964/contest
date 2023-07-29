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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        s: [chars; n],
    }
    for i in 0..=n - 9 {
        for j in 0..=m - 9 {
            let mut ok = true;
            for k in 0..3 {
                ok &= s[i + k][j..j + 3].iter().all(|&c| c == '#');
                ok &= s[i + 6 + k][j + 6..j + 9].iter().all(|&c| c == '#');
            }
            for k in 0..4 {
                ok &= s[i + k][j + 3] == '.';
                ok &= s[i + 3][j + k] == '.';
                ok &= s[i + 5][j + 5 + k] == '.';
                ok &= s[i + 5 + k][j + 5] == '.';
            }
            if ok {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
