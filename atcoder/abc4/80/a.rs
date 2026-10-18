fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let s1 = getline().trim().bytes().collect::<Vec<_>>();
    let s2 = getline().trim().bytes().collect::<Vec<_>>();
    if s1[0] + s2[1] == b'A' + b'D' {
        println!("No");
    } else {
        println!("Yes");
    }
}
