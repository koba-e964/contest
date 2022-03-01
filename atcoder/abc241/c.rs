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
        n: usize,
        s: [chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if i + 6 <= n {
                if (i..i + 6).filter(|&x| s[x][j] == '#').count() >= 4 {
                    println!("Yes");
                    return;
                }
            }
            if j + 6 <= n {
                if (j..j + 6).filter(|&x| s[i][x] == '#').count() >= 4 {
                    println!("Yes");
                    return;
                }
            }
            if i + 6 <= n && j + 6 <= n {
                if (0..6).filter(|&x| s[i + x][j + x] == '#').count() >= 4 {
                    println!("Yes");
                    return;
                }
            }
            if i + 6 <= n && j >= 5 {
                if (0..6).filter(|&x| s[i + x][j - x] == '#').count() >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
