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

fn one_step(s: Vec<char>) -> Vec<char> {
    let n = s.len();
    let mut ans = vec!['1'; n - 1];
    for i in 0..n - 1 {
        if s[i] == '1' && s[i + 1] == '1' {
            ans[i] = '0';
        }
    }
    ans
}

// https://yukicoder.me/problems/no/2037 (3)
// 2 段上に上ると、元々の列のうち、
// (i) 1 が 1 個だけあるところは 0 になる
// (ii) それ以外は同じ数字のまま
// が成り立つ。これを利用して実装できる。
fn main() {
    input! {
        n: usize, k: usize,
        s: chars,
    }
    let med = if (k + n) % 2 == 1 { n - 1 } else { n };
    let mut s = s;
    if med < n {
        s = one_step(s);
    }
    assert_eq!((k + med) % 2, 0);
    if k < med {
        for i in 1..s.len() - 1 {
            if s[i - 1..i + 2] == ['0', '1', '0'] {
                s[i] = '0';
            }
        }
    }
    let x = (med - k) / 2;
    let ans: String = s[x..s.len() - x].iter().cloned().collect();
    println!("{}", ans);
}
