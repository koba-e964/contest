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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut s = s;
    s.sort_by(|a, b| {
        let ax = a.chars().filter(|&c| c == 'X').count() as i64;
        let bx = b.chars().filter(|&c| c == 'X').count() as i64;
        let mut ac = 0;
        let mut bc = 0;
        for c in a.chars() {
            if c != 'X' {
                let idx = c as u8 - b'0';
                ac += idx as i64;
            }
        }
        for c in b.chars() {
            if c != 'X' {
                let idx = c as u8 - b'0';
                bc += idx as i64;
            }
        }
        (ac * bx).cmp(&(ax * bc))
    });
    let mut t = "".to_string();
    for s in s {
        t += &s;
    }
    let mut ans = 0i64;
    let mut x = 0i64;
    for c in t.chars() {
        if c == 'X' {
            x += 1;
            continue;
        }
        let idx = c as u8 - b'0';
        ans += idx as i64 * x;
    }
    println!("{}", ans);
}
