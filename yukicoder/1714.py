#!/usr/bin/env python3

h, w = map(int, input().split())
ni, nj = map(int, input().split())
zi, zj = map(int, input().split())

print('Yes' if (ni + nj + zi + zj) % 2 == 1 else 'No')
