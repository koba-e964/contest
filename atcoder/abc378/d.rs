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

fn rec(i: usize, j: usize, k: i32, s: &mut [Vec<char>]) -> i32 {
    let h = s.len();
    let w = s[0].len();
    if s[i][j] != '.' {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    s[i][j] = 'x';
    let mut ret = 0;
    for d in 0..4 {
        let ni = i as i64 + [0, 1, 0, -1][d];
        let nj = j as i64 + [1, 0, -1, 0][d];
        if ni >= 0 && ni < h as i64 && nj >= 0 && nj < w as i64 {
            ret += rec(ni as usize, nj as usize, k - 1, s);
        }
    }
    s[i][j] = '.';
    ret
}

fn main() {
    input! {
        h: usize, w: usize, k: i32,
        s: [chars; h],
    }
    let mut s = s;
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            ans += rec(i, j, k, &mut s);
        }
    }
    println!("{}", ans)
}
