fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let a = getline().chars().collect::<Vec<_>>();
    let b = getline().chars().collect::<Vec<_>>();
    for i in 0..a.len().min(b.len()) {
        if a[i] != b[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("0");
}
