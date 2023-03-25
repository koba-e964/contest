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
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut ans = s.clone();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] >= '0' && s[i][j] <= '9' {
                let r = (s[i][j] as u8 - b'0') as i32;
                for a in 0..h {
                    for b in 0..w {
                        let d = (a as i32 - i as i32).abs() + (b as i32 - j as i32).abs();
                        if d <= r {
                            ans[a][b] = '.';
                        }
                    }
                }
            }
        }
    }
    for i in 0..h {
        println!("{}", ans[i].iter().cloned().collect::<String>());
    }
}
