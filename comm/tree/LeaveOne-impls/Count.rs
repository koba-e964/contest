// Depends on: tree/Reroot.rs
// Verified by: https://atcoder.jp/contests/abc160/submissions/26509495
#[derive(Default, Clone, Debug)]
struct V<'a> {
    a: MInt,
    c: usize,
    fac: &'a [MInt],
    invfac: &'a [MInt],
}

impl<'a> LeaveOne for V<'a> {
    type T = (MInt, usize);
    type App = (&'a [MInt], &'a [MInt]);
    fn build(vals: &[Self::T], &(fac, invfac): &Self::App) -> Self {
        let mut a = MInt::new(1);
        let mut c = 1;
        for &(b, d) in vals {
            a *= b * invfac[d];
            c += d;
        }
        V {
            a: a * fac[c - 1],
            c: c,
            fac: fac,
            invfac: invfac,
        }
    }
    fn leave_one(&self, (b, d): Self::T) -> Self::T {
        (self.a * self.invfac[self.c - 1] * self.fac[d] * self.fac[self.c - 1 - d] * b.inv(), self.c - d)
    }
    fn exchange_one(&self, (b1, d1): Self::T, (b2, d2): Self::T) -> Self::T {
        (self.a * self.invfac[self.c - 1]
         * self.invfac[d2]
         * self.fac[d1]
         * self.fac[self.c - 1 + d2 - d1] * b2 * b1.inv(), self.c + d2 - d1)
    }
    fn add_one(&self, (b, d): Self::T) -> Self::T {
        (self.a * self.invfac[self.c - 1] * self.invfac[d] * self.fac[self.c - 1 + d] * b, self.c + d)
    }
    fn as_is(&self) -> Self::T {
        (self.a, self.c)
    }
}
