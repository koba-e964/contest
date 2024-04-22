fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline();
    for i in 0..s.len() - 2 {
        if &s[i..i + 3] == "ABC" {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
