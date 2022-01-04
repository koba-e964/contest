#!/usr/bin/env python3

m, k = map(int, input().split())

if m >= k + 2:
    w = m - 1 - (m - 1) // (k + 1)
    print('Win' if w % 2 == 1 else 'Lose')
    exit()

print('Win' if m % 2 == 0 else 'Draw')
