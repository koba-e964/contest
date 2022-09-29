use std::cmp::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut x = 1;
    let mut ans = 0;
    for c in getline().trim().chars() {
        if c == 'L' {
            x = max(x - 1, 1);
        } else {
            x = min(x + 1, 3);
        }
        if x == 3 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
