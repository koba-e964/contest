// https://twitter.com/noshi91/status/1200702280128856064
fn basis(a: &[i64]) -> Vec<i64> {
    let mut ret = vec![];
    for &(mut e) in a {
        for &b in &ret {
            e = std::cmp::min(e, e ^ b);
        }
        if e != 0 {
            ret.push(e);
        }
    }
    ret
}

fn main() {
    let a = vec![0x14, 0x28, 0x114];
    println!("{:?}", basis(&a));
}
