import json
import sys
import math

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
    with open(filename) as f:
        data = json.load(f)
    (parameters, bias) = ([math.pi], 0.0)
    print((parameters, bias))

    miss_rate = ValidateHalfMoonModel.simulate(
        validationVectors=data['Features'],
        validationLabels=data['Labels'],
        parameters=parameters, bias=bias
    )

    print(f"Miss rate: {miss_rate:0.2%}")
