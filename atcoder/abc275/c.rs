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
    input!(s: [chars; 9]);
    let mut ans = 0;
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] != '#' { continue; }
            for dx in 0..9 - i {
                for dy in 1..9 - j {
                    if s[i + dx][j + dy] != '#' { continue; }
                    if j < dx || i + dy >= 9 || s[i + dy][j - dx] != '#' { continue; }
                    if i + dx + dy >= 9 || j + dy < dx || j + dy - dx >= 9 || s[i + dx + dy][j + dy - dx] != '#' { continue; }
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
