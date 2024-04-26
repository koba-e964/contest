fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut c = 0;
    for (i, ch) in getline().trim().bytes().enumerate() {
        let idx = ch - b'A';
        c |= 1 << idx;
        if c == 7 {
            println!("{}", i + 1);
            return;
        }
    }
}
