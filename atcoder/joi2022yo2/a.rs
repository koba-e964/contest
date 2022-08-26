fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let q: usize = getline().trim().parse().unwrap();
    let mut v = vec![];
    for _ in 0..q {
        let l = getline().trim().to_string();
        if l == "READ" {
            println!("{}", v.pop().unwrap());
        } else {
            v.push(l);
        }
    }
}
