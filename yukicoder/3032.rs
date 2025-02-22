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

fn check(c: &str) -> bool {
    let mut x = 0;
    let mut y = 0;
    // 0: x = 0, y <= 0
    // 1: x = 0, y >= 1
    // 2: x >= 1
    // 3: x <= -1
    let mut sect = vec![0];
    for c in c.chars() {
        let last = sect[sect.len() - 1];
        let mut next = last;
        match c {
            'R' => {
                if x == 0 {
                    next = 2;
                }
                if x == -1 {
                    next = if y <= 0 { 0 } else { 1 };
                }
                x += 1;
            }
            'L' => {
                if x == 0 {
                    next = 3;
                }
                if x == 1 {
                    next = if y <= 0 { 0 } else { 1 };
                }
                x -= 1;
            }
            'U' => {
                if x == 0 && y == 0 {
                    next = 1;
                }
                y += 1;
            }
            'D' => {
                if x == 0 && y == 1 {
                    next = 0;
                }
                y -= 1;
            }
            _ => unreachable!(),
        }
        if last != next {
            sect.push(next);
        }
    }
    // Contracted to [0]?
    let mut st = vec![];
    for s in sect {
        if st.len() >= 2 && st[st.len() - 2] == s {
            st.pop();
        } else if st.len() == 0 || st[st.len() - 1] != s {
            st.push(s);
        }
    }
    st.len() == 1
}

// https://yukicoder.me/problems/no/3032 (3.5)
// The author read the editorial before implementing this.
// Tags: homotopy
fn main() {
    input! {
        n: usize, _m: usize,
        c: [String; n],
    }
    println!("{}", c.iter().filter(|&c| check(c)).count());
}
