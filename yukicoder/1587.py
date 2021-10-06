#!/usr/bin/env python3

n = int(input())

for i in range(n):
    for j in range(n):
        val = 0
        if j >= i:
            if i < n // 2 and i == j:
                val = 1
            else:
                val = 2
        print(val, end='')
    print()
