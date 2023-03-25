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
    let n = s.len();
    let mut dp = vec![0usize; n + 1];
    for i in 0..n {
        let x = (s[i] as u8 - b'0') as usize;
        dp[i + 1] = dp[i] ^ 1 << x;
    }
    let mut f = vec![0i64; 1024];
    for d in dp {
        f[d] += 1;
    }
    let mut ans = 0;
    for f in f {
        ans += f * (f - 1) / 2;
    }
    println!("{}", ans);
}
