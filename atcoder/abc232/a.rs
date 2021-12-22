fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let a: Vec<_> = getline().bytes().collect();
    println!("{}", (a[0] - 48) * (a[2] - 48));
}
