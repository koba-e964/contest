#!/usr/bin/env python3

k = int(input())
a = [[] for _ in range(4)]
a[0] = ['A', 'E']
a[1] = ['B']
a[2] = ['C']
a[3] = ['D']
for i in range(k):
    fr = i % 4
    to = (i + 1) % 4
    a[to].append(a[fr][0])
    a[fr] = a[fr][1:]

for i in range(4):
    print(''.join(a[i]))
