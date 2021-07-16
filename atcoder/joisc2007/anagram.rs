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

fn rec(f: &mut [i32], s: &[usize]) -> i64 {
    if s.is_empty() {
        return 1;
    }
    let mut ans = 0;
    for i in 0..s[0] {
        if f[i] == 0 {
            continue;
        }
        f[i] -= 1;
        let mut p = 1;
        for k in 1..s.len() {
            p *= k as i64;
        }
        for k in 0..26 {
            for l in 1..f[k] + 1 {
                p /= l as i64;
            }
        }
        ans += p;
        f[i] += 1;
    }
    f[s[0]] -= 1;
    ans + rec(f, &s[1..])
}

fn main() {
    input!(s: chars);
    let s: Vec<_> = s.iter().map(|&x| (x as u8 - b'A') as usize).collect();
    let mut f = vec![0; 26];
    for &c in &s {
        f[c] += 1;
    }
    println!("{}", rec(&mut f, &s));
}
