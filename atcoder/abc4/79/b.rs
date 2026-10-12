fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let s = getline();
    let mut ma = 0;
    let mut cur = 0;
    for c in s.chars() {
        if c == 'R' {
            cur += 1;
        } else {
            cur = 0;
        }
        ma = ma.max(cur);
    }
    println!("{ma}");
}
