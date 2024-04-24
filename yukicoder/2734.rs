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

const MOD: i64 = 998_244_353;

// https://yukicoder.me/problems/no/2734 (3)
// Tags: lexicographically-smallest-concatenation
fn main() {
    input! {
        n: usize,
        a: [chars; n],
    }
    let mut a = a;
    a.sort_by(|s, t| {
        let mut ss = s.clone();
        let mut tt = t.clone();
        ss.extend_from_slice(t);
        tt.extend_from_slice(s);
        ss.cmp(&tt)
    });
    let mut ans = 0;
    for s in a {
        for &c in &s {
            let dig = (c as u8 - b'0') as i64;
            ans = (ans * 10 + dig) % MOD;
        }
    }
    println!("{}", ans);
}
