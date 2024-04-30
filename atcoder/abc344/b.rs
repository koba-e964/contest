fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut a = vec![];
    loop {
        let s = getline();
        a.push(s.clone());
        if s == "0\n" {
            break;
        }
    }
    a.reverse();
    for a in a {
        print!("{}", a);
    }
}
