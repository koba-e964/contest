import time
import sys
from typing import List, Tuple, Dict, Any
import json
from matplotlib import pyplot
import numpy as np
import math


def read_from_file(filename: str) -> Tuple[List[Tuple[float, float]], List[float]]:
    with open(filename) as f:
        a = json.load(f)
        features = a['Features']
        labels = a['Labels']
        return (features, labels)


cases = [(0, 0), (0, 1), (1, 1), (1, 0)]
markers = [
    '.' if actual == classified else 'X'
    for (actual, classified) in cases
]
colors = ['blue', 'blue', 'red', 'red']


def plot_data(features: list, actual_labels: list, classified_labels: list = None, extra_lines: list = None):
    """Plots the data, labeling it with actual labels if there are no classification results provided,
    and with the classification results (indicating their correctness) if they are provided.
    """
    samples = np.array(features)
    pyplot.figure(figsize=(8, 8))
    for (idx_case, ((actual, classified), marker, color)) in enumerate(zip(cases, markers, colors)):
        mask = np.logical_and(np.equal(actual_labels, actual),
                              np.equal(actual if classified_labels == None else classified_labels, classified))
        if not np.any(mask):
            continue
        pyplot.scatter(
            samples[mask, 0], samples[mask, 1],
            label=f"Class {actual}" if classified_labels == None else f"Was {actual}, classified {classified}",
            marker=marker, s=300, c=[color],
        )
    # Add the lines to show the true classes boundaries, if provided
    if extra_lines != None:
        for line in extra_lines:
            pyplot.plot(line[0], line[1], color='gray')
    pyplot.legend()
    pyplot.show()


def separation_endpoint(angle: float) -> (float, float):
    if (angle < math.pi / 4):
        return (1, math.tan(angle))
    return (1 / math.tan(angle), 1)


filename = sys.argv[1]
(features, labels) = read_from_file(filename)


plot_data(features,
          labels)
