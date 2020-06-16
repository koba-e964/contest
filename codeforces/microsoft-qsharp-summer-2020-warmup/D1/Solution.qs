namespace Solution {
    open Microsoft.Quantum.MachineLearning;

    function ClassifierStructure() : ControlledRotation[] {
        return [
            ControlledRotation((0, new Int[0]), PauliY, 0)
        ];
    }

    operation Solve () : (ControlledRotation[], (Double[], Double)) {
        return (ClassifierStructure(), ([3.1245000000000003], 0.003500000000000003));
    }
}