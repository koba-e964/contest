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
    input! {
        n: usize,
        s: chars,
    }
    let mut t = vec![];
    let mut c = 0;
    for i in (0..n).rev() {
        let ch = s[i];
        if ch == '0' {
            t.push((i + 1, c));
            c = 0;
        } else {
            c += 1;
        }
    }
    t.push((0, c));
    t.reverse();
    let mut tot = 0i64;
    for (idx, len) in t {
        if idx > 0 {
            // [a, b) for a < idx - 1 < b <= idx + len
            tot += (idx as i64 - 1) * (len as i64 / 2 + 1);
            // [a, b) for a = idx - 1 < b <= idx + len
            tot += (len as i64 + 1) / 2;
        }
        // [a, b) for idx <= a < b <= idx + len
        let llen = len as i64;
        if len % 2 == 0 {
            tot += llen / 2 * (llen / 2 + 1);
        } else {
            tot += (llen + 1) * (llen + 1) / 4;
        }
    }
    println!("{}", tot);
}
