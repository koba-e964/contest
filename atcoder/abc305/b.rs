fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().bytes().collect();
    let mut x = (s[0] - b'A') as usize;
    let mut y = (s[2] - b'A') as usize;
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    let a = [3, 1, 4, 1, 5, 9];
    println!("{}", a[x..y].iter().sum::<i32>());
}
