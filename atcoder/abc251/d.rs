fn main() {
    let mut v = vec![];
    for i in 1..=100 {
        v.push(i);
        v.push(i * 100);
        v.push(i * 10000);
    }
    println!("{}", v.len());
    for i in 0..v.len() {
        print!("{}{}", v[i], if i + 1 == v.len() { "\n" } else { " " });
    }
}
