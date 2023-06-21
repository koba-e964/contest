#!/usr/bin/env sage
from sage.all import *

def check(u):
    k = u + 2
    var('y')
    x=(1-sqrt(1 - 4 * y))/2
    return ((1-(1-x)**k)/(1-x)**k).series(y,10)

for u in range(10):
    print(u, check(u))
