use std::cmp::*;
use std::collections::*;
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
    let n = parse(getline().trim());
    let mut hm: HashMap<String, i32, _> = HashMap::new();
    for _ in 0 .. n {
        let a: String = String::from(getline().trim());
        if let Some(x) = hm.get(&a).cloned() { // revised by https://twitter.com/mandel59/status/708973482411819008
            hm.insert(a, x + 1);
        } else {
            hm.insert(a, 1);
        }
    }
    println!("{}", if hm.values().all(|&x| x <= (n + 1) / 2) { "YES" } else { "NO" });
}
