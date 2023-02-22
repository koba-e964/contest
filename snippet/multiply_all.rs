// O(n log^2 n + args.len()) where n = sum(degs of args)
// Depends on: fft.rs, MInt.rs
// Verified by: ABC269-Ex (https://atcoder.jp/contests/abc269/submissions/39116328)
fn multiply_all(ops: &FPSOps<P>, args: Vec<Vec<MInt>>) -> Vec<MInt> {
    #[derive(Eq, PartialEq)]
    struct T(usize, Vec<MInt>);
    impl Ord for T {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0)
        }
    }
    impl PartialOrd for T {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut polys = BinaryHeap::new();
    for a in args {
        polys.push(T(a.len(), a));
    }
    let mut ans = vec![MInt::new(1)];
    while let Some(T(_, t1)) = polys.pop() {
        let t2 = if let Some(T(_, t)) = polys.pop() {
            t
        } else {
            ans = t1;
            break;
        };
        let tmp = ops.mul(t1, t2);
        polys.push(T(tmp.len(), tmp));
    }
    ans
}
