use std::cmp::min;
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

fn min_cww(s: &str, inf: usize) -> usize {
    let mut cc = 0;
    let mut wc = 0;
    let cs = s.chars();
    let mut i = 0;
    for c in cs {
        if c == 'c' {
            cc+=1;
        } else if c == 'w' && cc >= 1 {
            wc+=1;
        }
        if cc >= 1 && wc >= 2 {
            return i + 1;
        }
        i += 1;
    }
    return inf;
}

fn main() {
    let s = getline();
    let inf = 100000;
    let mut mi = inf;
    for i in 0 .. s.len() {
        let slice = &s[i..];
        mi = min(mi, min_cww(slice, inf));
    }
    println!("{}", if mi == inf { -1 } else { mi as i32});
}
