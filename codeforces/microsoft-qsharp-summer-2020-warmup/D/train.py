import json
import sys

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.colors as colors
import matplotlib.cm as cmx
plt.style.use('ggplot')

import qsharp
qsharp.packages.add("Microsoft.Quantum.MachineLearning::0.11.2006.403")
qsharp.reload()

# https://docs.microsoft.com/ja-jp/quantum/libraries/machine-learning/basic-classification?tabs=tabid-python

from Microsoft.Quantum.Samples import (
    TrainHalfMoonModel, ValidateHalfMoonModel, ClassifyHalfMoonModel
)

if __name__ == "__main__":
    filename = sys.argv[1]
    init = float(sys.argv[2])
    with open(filename) as f:
        data = json.load(f)
    parameter_starting_points = [
        [init],
    ]

    (parameters, bias) = TrainHalfMoonModel.simulate(
        trainingVectors=data['Features'],
        trainingLabels=data['Labels'],
        initialParameters=parameter_starting_points
    )
    print((parameters, bias))

    miss_rate = ValidateHalfMoonModel.simulate(
        validationVectors=data['Features'],
        validationLabels=data['Labels'],
        parameters=parameters, bias=bias
    )

    print(f"Miss rate: {miss_rate:0.2%}")
