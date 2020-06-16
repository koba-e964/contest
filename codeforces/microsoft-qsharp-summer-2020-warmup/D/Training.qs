namespace Microsoft.Quantum.Samples {
    open Microsoft.Quantum.Convert;
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Canon;
    open Microsoft.Quantum.Arrays;
    open Microsoft.Quantum.MachineLearning;
    open Microsoft.Quantum.Math;

    function Div(x: Double, y: Double): Double {
        return x / y;
    }

    function Normalize(sample : Double[]) : Double[] {
        let norm = Sqrt(sample[0] ^ 2.0 + sample[1] ^ 2.0);
        return Mapped(Div(_, norm), sample);
    }

    function Preprocessed(samples : Double[][]) : Double[][] {
        return Mapped(
            Normalize(_),
            samples
        );
    }

    function DefaultSchedule(samples : Double[][]) : SamplingSchedule {
        return SamplingSchedule([
            0..Length(samples) - 1
        ]);
    }

    function ClassifierStructure() : ControlledRotation[] {
        return [
            ControlledRotation((0, new Int[0]), PauliY, 0)
        ];
    }

    operation TrainHalfMoonModel(
        trainingVectors : Double[][],
        trainingLabels : Int[],
        initialParameters : Double[][]
    ) : (Double[], Double) {
        let samples = Mapped(
            LabeledSample,
            Zip(Preprocessed(trainingVectors), trainingLabels)
        );
        Message("Ready to train.");
        let (optimizedModel, nMisses) = TrainSequentialClassifier(
            Mapped(
                SequentialModel(ClassifierStructure(), _, 0.0),
                initialParameters
            ),
            samples,
            DefaultTrainingOptions()
                w/ LearningRate <- 2.0
                w/ MinibatchSize <- 20
                w/ Tolerance <- 0.005
                w/ NMeasurements <- 10000
                w/ MaxEpochs <- 10
                w/ VerboseMessage <- Message,
            DefaultSchedule(trainingVectors),
            DefaultSchedule(trainingVectors)
        );
        Message($"Training complete, found optimal parameters: {optimizedModel::Parameters}");
        Message($"nMisses = {nMisses}");
        return (optimizedModel::Parameters, optimizedModel::Bias);
    }

    operation ValidateHalfMoonModel(
        validationVectors : Double[][],
        validationLabels : Int[],
        parameters : Double[],
        bias : Double
    ) : Double {
        let samples = Mapped(
            LabeledSample,
            Zip(Preprocessed(validationVectors), validationLabels)
        );
        let tolerance = 0.005;
        let nMeasurements = 10000;
        let results = ValidateSequentialClassifier(
            SequentialModel(ClassifierStructure(), parameters, bias),
            samples,
            tolerance,
            nMeasurements,
            DefaultSchedule(validationVectors)
        );
        return IntAsDouble(results::NMisclassifications) / IntAsDouble(Length(samples));
    }

    operation ClassifyHalfMoonModel(
        samples : Double[][],
        parameters : Double[],
        bias : Double,
        tolerance  : Double,
        nMeasurements : Int
    )
    : Int[] {
        let model = Default<SequentialModel>()
            w/ Structure <- ClassifierStructure()
            w/ Parameters <- parameters
            w/ Bias <- bias;
        let features = Preprocessed(samples);
        let probabilities = EstimateClassificationProbabilities(
            tolerance, model,
            features, nMeasurements
        );
        return InferredLabels(model::Bias, probabilities);
    }

}