namespace Solution {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Math;

    operation Solve (unitary : (Qubit => Unit is Adj+Ctl)) : Int {
        mutable ans = -1;
        using (q = Qubit[2]) {
            H(q[0]);
            Controlled unitary([q[0]], q[1]);
            H(q[0]);
            if (M(q[0]) == One) {
                set ans = 1;
                X(q[0]);
            } else {
                set ans = 0;
            }
        }
        return ans;
    }
    operation TestZ(count: Int): Int {
        mutable agree = 0;
        for (test in 1..count) {
            let res = Solve(Z);
            if (res == 0) {
                set agree += 1;
            }
        }
        return agree;
    }
    operation TestmZ(count: Int): Int {
        mutable agree = 0;
        for (test in 1..count) {
            let res = Solve(minusZ);
            if (res == 1) {
                set agree += 1;
            }
        }
        return agree;
    }
    operation minusZ(q: Qubit): Unit is Adj + Ctl {
        Z(q);
        Rz(2.0 * PI(), q);
    }
}
