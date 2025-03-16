fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut orig = 0;
    let mut tot = 0;
    let mut last = 'o';
    for c in getline().trim().chars() {
        if c == last {
            tot += 1;
        }
        last = c;
        orig += 1;
        tot += 1;
    }
    println!("{}", (tot + 1) / 2 * 2 - orig);
}
