#!/usr/bin/env python3

n = int(input())

if n == 0:
    print('0')
    exit()

print('0.', end='')

for i in range(n):
    print([1, 4, 2, 8, 5, 7][i % 6], end='')

print()
