fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let f = getline();
    let a = f.trim().split_whitespace().collect::<Vec<_>>();
    let n = a.len();
    if (0..n - 2).any(|i| a[i] == a[i + 1] && a[i] == a[i + 2]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
