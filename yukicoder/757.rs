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

// Finds sum_{1 <= l < b^x} |l|. None if the result is >= b^n.
fn calc(b: i32, x: usize, n: usize) -> Option<Vec<i32>> {
    if x >= n + 1 {
        return None;
    }
    let mut ans = vec![0; n];
    for i in (0..x).rev() {
        let mut c = (i + 1) as i32 * (b - 1);
        let mut pos = n - i;
        while pos > 0 && c > 0 {
            ans[pos - 1] += c;
            c = ans[pos - 1] / b;
            ans[pos - 1] %= b;
            pos -= 1;
        }
        if pos == 0 && c != 0 {
            return None;
        }
    }
    Some(ans)
}

// Tags: multi-precision
fn main() {
    input! {
        b: i32,
        d: chars,
    }
    let n = d.len();
    let d: Vec<i32> = d.into_iter().map(|x| (x as u8 - b'0') as _).collect();
    let mut pass = 0;
    let mut fail = n + 1;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let res = calc(b, mid, n);
        if res.is_some() && res.unwrap() < d {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let lt = calc(b, pass, n).unwrap();
    let mut rem = d.clone();
    let mut c = 0;
    for i in (0..n).rev() {
        c = c + rem[i] - lt[i];
        let mut nc = 0;
        if c < 0 {
            c += b;
            nc -= 1;
        }
        rem[i] = c;
        c = nc;
    }
    // q = (rem - 1) / fail, r = (rem - 1) % fail
    // the (r+1)-st digit of (b^pass + q) is the answer.
    c = -1;
    for i in (0..n).rev() {
        c = rem[i] + c;
        let mut nc = 0;
        if c < 0 {
            c += b;
            nc -= 1;
        }
        rem[i] = c;
        c = nc;
    }
    c = 0;
    let mut quo = vec![0; n];
    for i in 0..n {
        c = b * c + rem[i];
        quo[i] = c / fail as i32;
        c %= fail as i32;
    }
    quo[n - fail] += 1;
    println!("{}", quo[n - fail + c as usize]);
}
