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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn rec(s: &[Vec<char>], pos: usize) -> i64 {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let mut ans = 0;
    let mut pass = s.len() + 1;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if s[mid - 1].len() <= pos {
            fail = mid;
        } else {
            pass = mid;
        }
    }
    let mut last = pass - 1;
    for c in 'a'..='z' {
        let mut pass = s.len() + 1;
        let mut fail = last;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if s[mid - 1].len() <= pos || s[mid - 1][pos] <= c {
                fail = mid;
            } else {
                pass = mid;
            }
        }
        ans += rec(&s[last..pass - 1], pos + 1);
        let len = (pass - last - 1) as i64;
        ans += len * (len - 1) / 2;
        last = pass - 1;
    }
    ans
}

// Tags: trie-less
fn solve() {
    input! {
        n: usize,
        s: [chars; n],
    }
    let mut s = s;
    s.sort();
    println!("{}", rec(&s, 0));
}
