// Depends on: tree/Reroot.rs
// Verified by: https://yukicoder.me/submissions/884415
#[derive(Default, Clone)]
struct NumeralConcatenation {
    // the multiplier that would be multiplied by the value of the parent node
    mul: MInt,
    // the sum in the current rooted tree
    add: MInt,
    // the info of the root node
    me: (MInt, MInt),
}

impl LeaveOne for NumeralConcatenation {
    type T = (MInt, MInt);
    type Me = (MInt, MInt);
    type App = ();
    fn build(me: Self::Me, vals: &[Self::T], &(): &Self::App) -> Self {
        let mut mul = MInt::new(0);
        let mut add = MInt::new(0);
        for &(b, d) in vals {
            mul += b;
            add += d;
        }
        NumeralConcatenation {
            mul: (mul + 1) * me.0,
            add: add + me.1 * (mul + 1),
            me: me,
        }
    }
    fn leave_one(&self, (b, d): Self::T) -> Self::T {
        (self.mul - self.me.0 * b, self.add - self.me.1 * b - d)
    }
    fn exchange_one(&self, (b1, d1): Self::T, (b2, d2): Self::T) -> Self::T {
        (self.mul - (b1 - b2) * self.me.0, self.add - (b1 - b2) * self.me.1 - d1 + d2)
    }
    fn add_one(&self, (b, d): Self::T) -> Self::T {
        (self.mul + self.me.0 * b, self.add + self.me.1 * b + d)
    }
    fn as_is(&self) -> Self::T {
        (self.mul, self.add)
    }
}
