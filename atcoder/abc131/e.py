#!/usr/bin/env python3

n, k = map(int, input().split())

lim = (n - 1) * (n - 2) // 2
if k > lim:
    print(-1)
    exit()

edges = []
for i in range(n - 1):
    edges.append((i, n - 1))

combs = []
for i in range(n - 1):
    for j in range(i):
        combs.append((j, i))

edges.extend(combs[:lim - k])

print(len(edges))
for a, b in edges:
    print(a + 1, b + 1)
