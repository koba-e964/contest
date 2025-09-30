fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let mut x = 0;
    while x < n && a[x] == 0 {
        x += 1;
    }
    let mut y = n;
    while y > 0 && a[y - 1] == 0 {
        y -= 1;
    }
    println!("{}", y.max(x + 1) - (x + 1));
}
