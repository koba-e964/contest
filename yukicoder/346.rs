use std::cmp::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn parse<T : std::str::FromStr>(s : &str) -> T {
    match s.parse::<T>() {
        Ok(t) => t,
        _    => panic!(),
    }
}

fn main() {
    let st = getline();
    let s = st.as_bytes();
    let mut sum = 0i64;
    let mut wc = 0i64;
    for i in (0 .. s.len()).rev() {
        let c = s[i] as char;
        if c == 'w' {
            wc += 1;
        } else if c == 'c' {
            sum += wc * (wc - 1) / 2;
        }
    }
    println!("{}", sum);
}
