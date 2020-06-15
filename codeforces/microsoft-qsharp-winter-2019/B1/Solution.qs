namespace Solution {
    open Microsoft.Quantum.Primitive;
    open Microsoft.Quantum.Arithmetic;
    open Microsoft.Quantum.Convert;
    open Microsoft.Quantum.Canon;
    open Microsoft.Quantum.Math;
    open Microsoft.Quantum.Diagnostics;
    open Microsoft.Quantum.Preparation;

    operation GetPsi0(qs: Qubit[]): Unit is Ctl + Adj {
        let N = Length(qs);
        if (N == 1) {
            X(qs[0]);
        } else {
            let fac = 1.0 / Sqrt(IntAsDouble(N));
            let angle = ArcSin(fac);
            Ry(2.0 * angle, qs[0]);
            (ControlledOnInt(0, GetPsi0))([qs[0]], qs[1..N - 1]);
        }
    }

    operation Solve (qs : Qubit[]) : Int {
        let angle = PI() * 2.0 / 3.0;
        Rz(-1.0 * angle, qs[1]);
        Rz(-2.0 * angle, qs[2]);
        Adjoint GetPsi0(qs);
        let s = MeasureInteger(LittleEndian(qs));
        if (s == 0) {
            return 0;
        } else {
            return 1;
        }
    }
    operation Test0(count: Int): Int {
        let fac = 1.0 / Sqrt(3.0);
        mutable agree = 0;
        for (_ in 0..count - 1) {
            using (qs = Qubit[3]) {
                let zero = ComplexPolar(0.0, 0.0);
                let coef = [
                    zero,
                    ComplexPolar(fac, 0.0),
                    ComplexPolar(fac, PI() * 2.0 / 3.0),
                    zero,
                    ComplexPolar(fac, PI() * 4.0 / 3.0)
                ];
                PrepareArbitraryState(coef, LittleEndian(qs));
                let ans = Solve(qs);
                ResetAll(qs);
                if (ans == 0) {
                    set agree += 1;
                }
            }
        }
        return agree;
    }
}
