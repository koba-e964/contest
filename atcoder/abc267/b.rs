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
    let r = [3, 2, 4, 1, 3, 5, 0, 2, 4, 6];
    let mut f = vec![0; 7];
    for i in 0..10 {
        if s[i] == '1' {
            f[r[i]] += 1;
        }
    }
    if s[0] != '0' {
        println!("No");
        return;
    }
    for i in 0..7 {
        for j in i + 1..7 {
            if f[i] == 0 || f[j] == 0 { continue; }
            for k in i + 1..j {
                if f[k] == 0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
