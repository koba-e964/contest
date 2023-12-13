// https://yukicoder.me/problems/no/2545 (3.5)
// https://nu50218.dev/posts/integer-division-by-multiplication/ の議論を使うことで、
// (x * 0x5555_5556) >> 32 が答えであることがわかる。
// x * 0x5555_5556 は常に 2^62 以下であるため問題ない。
fn main() {
    println!("67");

    // A[4] = 5x
    println!("plus 5 1 1");
    println!("plus 5 5 5");
    println!("plus 4 1 5");

    // A[4] = 0x55 * x
    println!("plus 5 4 4");
    println!("plus 5 5 5");
    println!("plus 5 5 5");
    println!("plus 5 5 5");
    println!("plus 4 4 5");

    // A[4] = 0x5555 * x
    println!("plus 5 4 4");
    for _ in 1..8 {
        println!("plus 5 5 5");
    }
    println!("plus 4 4 5");

    // A[4] = 0x55555555 * x
    println!("plus 5 4 4");
    for _ in 1..16 {
        println!("plus 5 5 5");
    }
    println!("plus 4 4 5");

    // A[3] = 0x55555556 * x
    println!("plus 3 4 1");

    for _ in 0..32 {
        println!("div 3 3");
    }
}
