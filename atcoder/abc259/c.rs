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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(s: String) -> Vec<(char, u32)> {
    let mut ans = vec![];
    let mut last = '{';
    let mut num = 0;
    for c in s.chars() {
        if last == c {
            num += 1;
        } else {
            ans.push((last, num));
            last = c;
            num = 1;
        }
    }
    ans.push((last, num));
    ans
}

fn main() {
    input! {
        s: String,
        t: String,
    }
    let x = calc(s);
    let y = calc(t);
    if x.len() != y.len() {
        println!("No");
        return;
    }
    for i in 0..x.len() {
        if x[i].0 != y[i].0 || x[i].1 > y[i].1 || (x[i] != y[i] && x[i].1 == 1) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
