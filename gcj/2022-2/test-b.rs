mod inner {
    include!("b.rs");
}

fn main() {
    let tbl = inner::precomp(1001);
    for r in 1..1001 {
        let naive = inner::naive(r);
        let ans = inner::calc(r, &tbl);
        assert_eq!(naive, ans, "r = {}", r);
    }
}
