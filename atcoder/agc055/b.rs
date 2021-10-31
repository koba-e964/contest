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

fn red(s: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut s = s.to_vec();
    for i in 0..n {
        s[i] = (s[i] + 3 - i % 3) % 3;
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(s[i]);
        let l = ans.len();
        if l >= 3 && ans[l - 2] == ans[l - 1] && ans[l - 3] == ans[l - 1] {
            for _ in 0..3 {
                ans.pop();
            }
        }
    }
    ans
}

fn main() {
    input! {
        n: usize,
        s: chars,
        t: chars,
    }
    let getind = |c| (c as u8 - b'A') as usize;
    if s == t {
        println!("YES");
        return;
    }
    let s: Vec<usize> = s.into_iter().map(getind).collect();
    let t: Vec<usize> = t.into_iter().map(getind).collect();
    let mut accs = vec![[0; 3]; n + 1];
    let mut acct = vec![[0; 3]; n + 1];
    for i in 0..n {
        accs[i + 1] = accs[i];
        accs[i + 1][s[i]] += 1;
        acct[i + 1] = acct[i];
        acct[i + 1][t[i]] += 1;
    }
    let sr = red(&s);
    let tr = red(&t);
    println!("{}", if sr == tr { "YES" } else { "NO" });
}
