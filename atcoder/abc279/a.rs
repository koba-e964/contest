fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut a = 0;
    for c in getline().trim().chars() {
        if c == 'w' {
            a += 2;
        } else {
            a += 1;
        }
    }
    println!("{}", a);
}
