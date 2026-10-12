fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn ask(x: usize, y: usize) -> bool {
    if x >= y {
        return true;
    }
    println!("? {} {}", x + 1, y + 1);
    let s = getline().trim().to_string();
    s == "Yes"
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut pos = 1;
    let mut ans = 0i64;
    for i in 0..n {
        while pos < n && ask(i, pos) {
            pos += 1;
        }
        ans += (pos - i) as i64 - 1;
    }
    println!("! {ans}");
}
