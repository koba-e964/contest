#!/usr/bin/env sage -python
from sage.all import *

def calc(n: int, i: int, j: int) -> int:
    p = []
    for k in range(i // 2):
        p.append((k + 1, i - k))
    q = []
    for k in range(j // 2):
        q.append((n + 1 - (k + 1), n + 1 - (j - k)))
    g = PermutationGroup([p, q])
    return g.order()


n = 120

for v in [10, 100, 1000]:
    print(calc(v, v - 1, v - 2))

seen = {}
for i in range(1, n + 1):
    p = []
    for j in range(1, n + 1):
        seen[calc(n, i, j)] = (i, j)
print(f'{n} orders = {seen}')
