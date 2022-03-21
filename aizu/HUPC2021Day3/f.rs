use std::io::{Write, BufWriter};
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(s: chars);
    let n = s.len();
    let mut imos = vec![0; n + 1];
    let mut st = vec![];
    for i in 0..n {
        if '0' <= s[i] && s[i] <= '9' {
            let x = (s[i] as u8 - b'0') as i64;
            imos[i + 1] += x;
            st.push(x);
        }
        if s[i] == ')' {
            let x = st.pop().unwrap();
            imos[i] -= x;
        }
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    let mut ans = vec![];
    for i in 0..n {
        let c = s[i];
        if c < 'a' || c > 'z' { continue; }
        let x = ((c as u8 - b'a') as i64 + imos[i]) % 26;
        ans.push((b'a' + x as u8) as char);
    }
    puts!("{}\n", ans.into_iter().collect::<String>());
}
